use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;
use windows_sys::Win32::Foundation::HMODULE;
use windows_sys::Win32::System::LibraryLoader::{GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, GetModuleHandleExW, GetModuleFileNameW};

/// Global to cache the DLL's HMODULE. Set this in DllMain.
static mut DLL_HMODULE: HMODULE = 0;

/// Call this from DllMain with the hinstDLL parameter.
pub unsafe fn set_dll_hmodule(hmodule: HMODULE) {
    unsafe{DLL_HMODULE = hmodule};
}

/// Retrieve the full path of the DLL this code is compiled into.
fn get_dll_path() -> Option<PathBuf> {
    let mut buffer = vec![0u16; 260]; // MAX_PATH

    let hmodule = unsafe {
        // Alternatively, use DLL_HMODULE if set via DllMain
        let mut hm: HMODULE = 0;
        let result = GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            get_dll_path as *const _,  // any address within this DLL
            &mut hm,
        );
        if result == 0 {
            return None;
        }
        hm
    };

    let len = unsafe { GetModuleFileNameW(hmodule, buffer.as_mut_ptr(), buffer.len() as u32) };
    if len == 0 {
        return None;
    }

    let os_string = OsString::from_wide(&buffer[..len as usize]);
    Some(PathBuf::from(os_string))
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::log::log(&format!($($arg)*))
    };
}

/// Write a message to a log file next to the DLL.
/// The log file is named after the DLL, e.g. "mylib.log".
pub fn log(message: &str) {
    let dll_path = get_dll_path().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Could not determine DLL path")
    }).unwrap();

    // Build the log path: same directory, same stem, ".log" extension
    let log_path = dll_path.with_extension("log");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .unwrap();

    let timestamp = get_timestamp();
    let mod_name = dll_path.file_stem().unwrap().to_str().unwrap();
    let _ = writeln!(file, "[{mod_name} {timestamp}] {message}");
    eprintln!("[{mod_name} {timestamp}] {message}");
}

/// Simple timestamp using Windows SYSTEMTIME.
fn get_timestamp() -> String {
    use windows_sys::Win32::Foundation::SYSTEMTIME;
    use windows_sys::Win32::System::SystemInformation::GetLocalTime;
    let mut st = SYSTEMTIME {
        wYear: 0, wMonth: 0, wDayOfWeek: 0, wDay: 0,
        wHour: 0, wMinute: 0, wSecond: 0, wMilliseconds: 0,
    };
    unsafe { GetLocalTime(&mut st); }
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        st.wYear, st.wMonth, st.wDay,
        st.wHour, st.wMinute, st.wSecond, st.wMilliseconds
    )
}