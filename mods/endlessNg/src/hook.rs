use std::mem;
use std::sync::OnceLock;
use core::ffi::c_void;
use eldenring::{
    param::EQUIP_PARAM_GOODS_ST
};
use winhook::HookInstaller;
use pelite::pattern::parse;
use crate::patch::*;
use crate::log;

const GET_GOODS_AOB: &str = "8b 12 48 8d 4d c0 83 cf ff 89 75 c4 89 7d c0 48 89 75 c8 e8 ? ? ? ? 48 8b 45 c8 48 85 c0 74 03 8b 78 74 8b c7 eb 32";
const GET_GOODS_OFFSET: usize = 20;

#[repr(C, packed)]
struct GetGoodsResult {
    pub id: u32,
    pub _1: [u8; 0x4],
    pub row: *mut EQUIP_PARAM_GOODS_ST,
}

type GetGoodsType = unsafe extern "C" fn(
    result: *mut GetGoodsResult,
    id: u32,
);

pub static GET_GOODS_ORIGINAL_HOLDER: OnceLock<GetGoodsType> = OnceLock::new();

pub fn init_hooks() {
    let Some(address) = aob_scan(&parse(GET_GOODS_AOB).unwrap()) else {return;};
    unsafe {
        let address = address.wrapping_add(GET_GOODS_OFFSET);
        let address = (address as isize)
            .wrapping_add(mem::size_of::<i32>() as isize)
            .wrapping_add(*(address as *mut i32) as isize)
            as *mut GetGoodsType;
        GET_GOODS_ORIGINAL_HOLDER.set(*address).ok();
    }
}