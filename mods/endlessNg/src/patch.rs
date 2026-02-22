use pelite::pe64::{PeView, Pe};
use pelite::pattern::{Atom, parse};
use std::ffi::c_void;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

pub fn aob_scan(pattern: &[Atom]) -> Option<*mut u8> {
    unsafe {
        // get module base
        let hmodule = GetModuleHandleA(None).ok()?;
        let base = hmodule.0 as *const u8;

        // create a PE view
        let pe = PeView::module(base);

        // create a scanner for this PE
        let scanner = pe.scanner();

        // find first match in code section
        let mut save = [0u32; 16]; // space for captures if needed
        if scanner.finds_code(pattern, &mut save) {
            let rva = save[0];
            return Some((base as usize + rva as usize) as *mut u8);
        }
    }

    None
}

use std::ptr;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS};

unsafe fn replace_expected_bytes(
    address: *mut u8,
    expected: &[u8],
    new_bytes: &[u8],
) -> bool {
    let existing = unsafe {std::slice::from_raw_parts(address, expected.len())};

    if existing != expected {
        eprintln!("Bytes do not match!");
        return false;
    }

    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    unsafe {let _ = VirtualProtect(
        address as *const c_void,
        new_bytes.len(),
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    );}

    unsafe  { ptr::copy_nonoverlapping(new_bytes.as_ptr(), address, new_bytes.len()); }

    true
}

fn string_to_bytes(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|b| u8::from_str_radix(b, 16).expect("Invalid hex byte"))
        .collect()
}

pub fn perform_patch(
    aob_str: &str,
    expected_str: &str,
    new_bytes_str: &str,
    offset: usize,
) {
    let aob = parse(aob_str).unwrap();
    let expected = string_to_bytes(expected_str);
    let new_bytes = string_to_bytes(new_bytes_str);

    if let Some(addr) = aob_scan(&aob) {
        unsafe {
            let patch_addr = addr.add(offset);
            if replace_expected_bytes(patch_addr, &expected, &new_bytes) {
                eprintln!("Patch applied at {:p}", patch_addr);
            } else {
                eprintln!("Patch failed (verification mismatch) at {:p}", patch_addr);
            }
        }
    } else {
        eprintln!("Signature not found");
    }
}