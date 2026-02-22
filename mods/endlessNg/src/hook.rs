use std::mem;
use std::sync::OnceLock;
use eldenring::{
    param::EQUIP_PARAM_GOODS_ST
};
use winhook::{HookHandle, HookInstaller};
use pelite::pattern::parse;
use crate::patch::*;

const GET_GOODS_AOB: &str = "8b 12 48 8d 4d c0 83 cf ff 89 75 c4 89 7d c0 48 89 75 c8 e8 ? ? ? ? 48 8b 45 c8 48 85 c0 74 03 8b 78 74 8b c7 eb 32";
const GET_GOODS_OFFSET: usize = 20;

#[repr(C, packed)]
pub struct GetGoodsResult {
    pub id: u32,
    pub _1: [u8; 0x4],
    pub row: *mut EQUIP_PARAM_GOODS_ST,
}

pub type GetGoodsType = unsafe extern "C" fn(
    result: *mut GetGoodsResult,
    id: u32,
);

pub static GET_GOODS_ORIGINAL_HOLDER: OnceLock<GetGoodsType> = OnceLock::new();

pub fn init_hooks() -> Option<HookHandle> {
    let Some(address) = aob_scan(&parse(GET_GOODS_AOB).unwrap()) else {return None};
    unsafe {
        let address = address.wrapping_add(GET_GOODS_OFFSET);
        let address = (address as isize)
            .wrapping_add(mem::size_of::<i32>() as isize)
            .wrapping_add(*(address as *mut i32) as isize)
            as *mut GetGoodsType;
        let func: GetGoodsType = std::mem::transmute(address);
        GET_GOODS_ORIGINAL_HOLDER.set(func).ok();
    }

    let Some(get_goods_original) = GET_GOODS_ORIGINAL_HOLDER.get() else {return None};
    let hook_handle = HookInstaller::<GetGoodsType>::for_function(*get_goods_original)
    .install_mut({
        move |original| move |result, id| {
            if id == 67350{
                unsafe { 
                    original(result, 2912);
                    (*(*result).row).set_max_num(9999);
                }
            }
            else {
                unsafe { original(result, id) };
            }
        }
    })
    .unwrap();

    unsafe {hook_handle.enable(true)};

    return Some(hook_handle);
}