use closure_ffi::traits::FnPtr;
use std::mem;
use std::sync::OnceLock;
use winhook::{CConv, HookInstaller};
use pelite::pattern::parse;
use crate::patch::*;
use crate::log;

pub fn get_addr_from_call (address: usize) -> usize {
    unsafe {
        let address = (address as isize)
            .wrapping_add(mem::size_of::<i32>() as isize)
            .wrapping_add(std::ptr::read_unaligned(address as *const i32) as isize)
            as usize;
        return address;
    }
}

pub fn make_installer_from_call_aob<T: FnPtr + CConv + 'static + std::fmt::Pointer> (aob: &str, offset: usize, holder: &OnceLock::<T>) -> Option::<HookInstaller::<T>> {
    let Some(address) = aob_scan(&parse(aob).unwrap()) else {return None};
    unsafe {
        let address = address.wrapping_add(offset);
        let address = get_addr_from_call(address as usize) as *mut T;
        let func: T = std::mem::transmute_copy::<*mut T, T>(&address);
        holder.set(func).ok();
    }
    let Some(func) = holder.get() else {return None};
    let hook = HookInstaller::<T>::for_function(*func);
    log!("setup hook installer of func {:p}", *func);
    return Some(hook);
}