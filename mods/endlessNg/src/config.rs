#![allow(dead_code)]

use std::ffi::OsString;
use std::fs;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use toml_edit::{DocumentMut, Item, Table, Value};
use windows_sys::Win32::Foundation::HMODULE;
use windows_sys::Win32::System::LibraryLoader::{
    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
    GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
    GetModuleHandleExW, GetModuleFileNameW,
};

// ---------------------------------------------------------------------------
// Public schema types
// ---------------------------------------------------------------------------

/// The typed default value for a config field.
#[derive(Debug, Clone)]
pub enum DefaultValue {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl DefaultValue {
    fn to_toml_value(&self) -> Value {
        match self {
            DefaultValue::Str(s)   => Value::from(s.as_str()),
            DefaultValue::Int(i)   => Value::from(*i),
            DefaultValue::Float(f) => Value::from(*f),
            DefaultValue::Bool(b)  => Value::from(*b),
        }
    }
}

// Convenience conversions so callers can write `"hello".into()`, `42.into()`, etc.
impl From<&str>   for DefaultValue { fn from(v: &str)   -> Self { DefaultValue::Str(v.to_owned()) } }
impl From<String> for DefaultValue { fn from(v: String) -> Self { DefaultValue::Str(v) } }
impl From<i64>    for DefaultValue { fn from(v: i64)    -> Self { DefaultValue::Int(v) } }
impl From<i32>    for DefaultValue { fn from(v: i32)    -> Self { DefaultValue::Int(v as i64) } }
impl From<f64>    for DefaultValue { fn from(v: f64)    -> Self { DefaultValue::Float(v) } }
impl From<bool>   for DefaultValue { fn from(v: bool)   -> Self { DefaultValue::Bool(v) } }

/// A single field in the config schema.
#[derive(Debug, Clone)]
pub struct SchemaField {
    /// TOML section (table) name, e.g. `"network"`.
    pub section: String,
    /// Key name within that section, e.g. `"port"`.
    pub key: String,
    /// Value written when the key is absent from the file.
    pub default: DefaultValue,
    /// Optional comment placed after the value on the same line.
    pub comment: Option<String>,
}

impl SchemaField {
    pub fn new(
        section: impl Into<String>,
        key:     impl Into<String>,
        default: impl Into<DefaultValue>,
        comment: impl Into<Option<String>>,
    ) -> Self {
        SchemaField {
            section: section.into(),
            key:     key.into(),
            default: default.into(),
            comment: comment.into(),
        }
    }
}

/// Builder-style schema description.
///
/// ```rust
/// let schema = Schema::new()
///     .field("general", "log_level",  "info",  Some("trace|debug|info|warn|error"))
///     .field("general", "enabled",    true,     None)
///     .field("network", "port",       8080_i64, Some("TCP port"));
///
/// dll_config::init(schema)?;
/// ```
#[derive(Debug, Default, Clone)]
pub struct Schema(pub Vec<SchemaField>);

impl Schema {
    pub fn new() -> Self { Schema(Vec::new()) }

    pub fn field<S>(
        mut self,
        section: impl Into<String>,
        key: impl Into<String>,
        default: impl Into<DefaultValue>,
        comment: Option<S>,
    ) -> Self
    where
        S: Into<String>,
    {
        let comment = comment.map(Into::into);

        self.0.push(SchemaField::new(section, key, default, comment));
        self
    }
}

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

struct Config {
    doc:  DocumentMut,
    path: PathBuf,
}

// ---------------------------------------------------------------------------
// Windows helper – locate the DLL on disk
// ---------------------------------------------------------------------------

/// Returns the full path of the DLL that contains this code.
fn dll_path() -> PathBuf {
    unsafe {
        let mut hmod: HMODULE = 0;
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
                | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            dll_path as *const _,
            &mut hmod,
        );
        let mut buf = vec![0u16; 32768];
        let len = GetModuleFileNameW(hmod, buf.as_mut_ptr(), buf.len() as u32) as usize;
        buf.truncate(len);
        PathBuf::from(OsString::from_wide(&buf))
    }
}

fn config_path() -> PathBuf {
    dll_path().with_extension("toml")
}

// ---------------------------------------------------------------------------
// Core logic
// ---------------------------------------------------------------------------

fn load_or_create_config(path: &PathBuf, schema: &Schema) -> Result<DocumentMut, String> {
    let raw = if path.exists() {
        fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?
    } else {
        String::new()
    };

    let mut doc: DocumentMut = raw
        .parse::<DocumentMut>()
        .map_err(|e| format!("parse error: {e}"))?;

    let mut dirty = !path.exists();

    for field in &schema.0 {
        // Ensure the section table exists.
        if !doc.contains_table(&field.section) {
            doc[&field.section] = toml_edit::table();
            dirty = true;
        }

        let section: &mut Table = doc[&field.section]
            .as_table_mut()
            .expect("section must be a table");

        if !section.contains_key(&field.key) {
            let mut item = Item::Value(field.default.to_toml_value());

            if let Some(comment) = &field.comment {
                item.as_value_mut()
                    .unwrap()
                    .decor_mut()
                    .set_suffix(format!(" # {comment}"));
            }

            section.insert(&field.key, item);
            dirty = true;
        }
    }

    if dirty {
        fs::write(path, doc.to_string()).map_err(|e| format!("write error: {e}"))?;
    }

    Ok(doc)
}

// ---------------------------------------------------------------------------
// Public Rust API
// ---------------------------------------------------------------------------

/// Initialise the config subsystem with a caller-supplied schema.
///
/// Must be called before any `get_*` / `set_*` functions.  Subsequent calls
/// are silently ignored (first call wins).
pub fn init(schema: Schema) -> Result<(), String> {
    if CONFIG.get().is_some() {
        return Ok(());
    }
    let path = config_path();
    let doc  = load_or_create_config(&path, &schema)?;
    // OnceLock::set returns Err when already set (harmless race) – ignore it.
    let _ = CONFIG.set(Mutex::new(Config { doc, path }));
    Ok(())
}

/// Retrieve a string value.
pub fn get_str(section: &str, key: &str) -> Option<String> {
    let guard = CONFIG.get()?.lock().ok()?;
    guard.doc[section][key].as_str().map(|s| s.to_owned())
}

/// Retrieve an integer value.
pub fn get_int(section: &str, key: &str) -> Option<i64> {
    let guard = CONFIG.get()?.lock().ok()?;
    guard.doc[section][key].as_integer()
}

/// Retrieve a float value.
pub fn get_float(section: &str, key: &str) -> Option<f64> {
    let guard = CONFIG.get()?.lock().ok()?;
    guard.doc[section][key].as_float()
}

/// Retrieve a boolean value.
pub fn get_bool(section: &str, key: &str) -> Option<bool> {
    let guard = CONFIG.get()?.lock().ok()?;
    guard.doc[section][key].as_bool()
}

/// Write a string value and flush to disk.
pub fn set_str(section: &str, key: &str, value: &str) -> Result<(), String> {
    let mut g = CONFIG.get().ok_or("Config not initialised")?.lock().map_err(|e| e.to_string())?;
    g.doc[section][key] = toml_edit::value(value);
    let p = g.path.clone();
    fs::write(&p, g.doc.to_string()).map_err(|e| e.to_string())
}

/// Write an integer value and flush to disk.
pub fn set_int(section: &str, key: &str, value: i64) -> Result<(), String> {
    let mut g = CONFIG.get().ok_or("Config not initialised")?.lock().map_err(|e| e.to_string())?;
    g.doc[section][key] = toml_edit::value(value);
    let p = g.path.clone();
    fs::write(&p, g.doc.to_string()).map_err(|e| e.to_string())
}

/// Write a boolean value and flush to disk.
pub fn set_bool(section: &str, key: &str, value: bool) -> Result<(), String> {
    let mut g = CONFIG.get().ok_or("Config not initialised")?.lock().map_err(|e| e.to_string())?;
    g.doc[section][key] = toml_edit::value(value);
    let p = g.path.clone();
    fs::write(&p, g.doc.to_string()).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// DllMain
//
// DllMain is intentionally a no-op here. Because the schema is defined by
// the caller, initialisation cannot happen inside DllMain. Instead, export
// a dedicated init function (see `plugin_init` example below) and have the
// host call it after loading the DLL, or call `init()` lazily on first use.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Example: exported Rust-side init wrapper
//
// In your own DLL crate, re-export something like this so the host can
// trigger initialisation with a known schema at the right moment:
//
//   #[no_mangle]
//   pub extern "C" fn plugin_init() -> i32 {
//       let schema = Schema::new()
//           .field("general", "log_level", "info", Some("verbosity"))
//           .field("network", "port",      8080_i64, None);
//       match dll_config::init(schema) {
//           Ok(_)  => 1,
//           Err(e) => { eprintln!("{e}"); 0 }
//       }
//   }
// ---------------------------------------------------------------------------