use closure_ffi::traits::FnPtr;
use windows::core::w;
use windows_sys::core::PCWSTR;
use std::mem;
use std::sync::OnceLock;
use eldenring::{
    param::EQUIP_PARAM_GOODS_ST
};
use winhook::{CConv, HookHandle, HookInstaller};
use pelite::pattern::parse;
use crate::patch::*;

pub const GET_GOODS_AOB: &str = "8b 12 48 8d 4d c0 83 cf ff 89 75 c4 89 7d c0 48 89 75 c8 e8 ? ? ? ? 48 8b 45 c8 48 85 c0 74 03 8b 78 74 8b c7 eb 32";
pub const GET_GOODS_OFFSET: usize = 20;

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
pub static INTENSITY_DISPLAY_GOODS: OnceLock<EQUIP_PARAM_GOODS_ST> = OnceLock::new();
pub static INTENSITY_CHANGE_GOODS: OnceLock<EQUIP_PARAM_GOODS_ST> = OnceLock::new();

pub const GET_MESSAGE_AOB: &str = "75 24 44 8b cb 33 d2 41 b8 9c 01 00 00 48 8b cf e8 ? ? ? ? 48 85 c0 48 8d 0d ? ? ? ? 48 0f 45 c8 48 8b c1 48 8b 5c 24 30";
pub const GET_MESSAGE_OFFSET: usize = 17;

pub type GetMessageType = unsafe extern "C" fn(
    message_repository: usize,
    _1: u32,
    msg_bnd: u32,
    msg_id: u32,
) -> PCWSTR;

pub static GET_MESSAGE_ORIGINAL_HOLDER: OnceLock<GetMessageType> = OnceLock::new();

pub fn get_addr_from_call (address: usize) -> usize {
    unsafe {
        let address = (address as isize)
            .wrapping_add(mem::size_of::<i32>() as isize)
            .wrapping_add(std::ptr::read_unaligned(address as *const i32) as isize)
            as usize;
        return address;
    }
}

pub fn make_installer_from_call_aob<T: FnPtr + CConv + 'static> (aob: &str, offset: usize, holder: &OnceLock::<T>) -> Option::<HookInstaller::<T>> {
    let Some(address) = aob_scan(&parse(aob).unwrap()) else {return None};
    unsafe {
        let address = address.wrapping_add(offset);
        let address = get_addr_from_call(address as usize) as *mut T;
        let func: T = std::mem::transmute_copy::<*mut T, T>(&address);
        holder.set(func).ok();
    }
    let Some(func) = holder.get() else {return None};
    let hook = HookInstaller::<T>::for_function(*func);
    return Some(hook);
}

pub fn init_hooks() -> Vec<HookHandle> {
    let hook_handle_goods = make_installer_from_call_aob::<GetGoodsType>(GET_GOODS_AOB, GET_GOODS_OFFSET, &GET_GOODS_ORIGINAL_HOLDER).unwrap()
    .install_mut({
        move |original| move |result, id| {
            if id == 67350{
                unsafe {
                    original(result, 2912);
                    if INTENSITY_DISPLAY_GOODS.get().is_none() {
                        let mut goods = (*(*result).row).clone();
                        goods.set_max_num(9999);
                        INTENSITY_DISPLAY_GOODS.set(goods).unwrap();
                    }
                    (*result).id = 67350;
                    (*result).row = INTENSITY_DISPLAY_GOODS.get().unwrap() as *const _ as *mut _;
                }
            } else if id == 67351 {
                unsafe {
                    original(result, 2912);
                    if INTENSITY_CHANGE_GOODS.get().is_none() {
                        let mut goods = (*(*result).row).clone();
                        goods.set_max_num(1);
                        goods.set_is_drop(0);
                        goods.set_is_discard(0);
                        goods.set_is_consume(0);
                        goods.set_is_deposit(0);
                        goods.set_max_repository_num(0);
                        goods.set_rarity(5);
                        goods.set_sell_value(-1);
                        goods.set_sort_id(349);
                        goods.set_sort_group_id(10);
                        let grace_memory_icon_id = 9;
                        goods.set_icon_id(grace_memory_icon_id);
                        INTENSITY_CHANGE_GOODS.set(goods).unwrap();
                    }
                    (*result).id = 67351;
                    (*result).row = INTENSITY_CHANGE_GOODS.get().unwrap() as *const _ as *mut _;
                }
            } else {
                unsafe { original(result, id) };
            }
        }
    }).unwrap();

    let hook_handle_message = make_installer_from_call_aob::<GetMessageType>(GET_MESSAGE_AOB, GET_MESSAGE_OFFSET, &GET_MESSAGE_ORIGINAL_HOLDER).unwrap()
    .install_mut({
        move |original| move |message_repository, _1, msg_bnd, msg_id| {
            if msg_bnd == 10 {
                if msg_id == 67350 {
                    return w!("Modify Intensity By:").as_ptr();
                }
                if msg_id == 67351 {
                    return w!("Grace Ascetic").as_ptr();
                }
            }
            if msg_bnd == 20 {
                if msg_id == 67351 {
                    return w!("Grace Ascetic Info").as_ptr();
                }
            }
            if msg_bnd == 24 {
                if msg_id == 67351 {
                    return w!("Grace Ascetic Caption").as_ptr();
                }
            }
            if msg_bnd == 33 {
                if msg_id == 22021100 {
                    return w!("Increase Intensity (Current: <?loopCount?>)").as_ptr();
                }
                if msg_id == 22021101 {
                    return w!("Decrease Intensity (Current: <?loopCount?>)").as_ptr();
                }
                if msg_id == 22021102 {
                    return w!("Current Intensity: <?loopCount?>").as_ptr();
                }
                if msg_id == 22021103 {
                    return w!("Intensity Updated").as_ptr();
                }
            }
            return unsafe { original(message_repository, _1, msg_bnd, msg_id) };
        }
    }).unwrap();

    unsafe {
        hook_handle_goods.enable(true);
        hook_handle_message.enable(true);
    };

    return vec![hook_handle_goods, hook_handle_message];
}