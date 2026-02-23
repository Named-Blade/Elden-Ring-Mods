use std::sync::OnceLock;
use std::collections::HashMap;
use windows::core::w;
use windows::core::{PCWSTR, HSTRING};
use winhook::HookHandle;

use crate::hook::*;

const GET_MESSAGE_AOB: &str = "75 24 44 8b cb 33 d2 41 b8 9c 01 00 00 48 8b cf e8 ? ? ? ? 48 85 c0 48 8d 0d ? ? ? ? 48 0f 45 c8 48 8b c1 48 8b 5c 24 30";
const GET_MESSAGE_OFFSET: usize = 17;

type GetMessageType = unsafe extern "C" fn(
    message_repository: usize,
    _1: u32,
    msg_bnd: u32,
    msg_id: u32,
) -> PCWSTR;

static GET_MESSAGE_ORIGINAL_HOLDER: OnceLock<GetMessageType> = OnceLock::new();

pub struct MessageData {
    map: HashMap<u32, HashMap<u32, Box<PCWSTR>>>,
    hook: HookHandle
}

impl MessageData {
    pub fn add_message(&mut self, msg_bnd: u32, msg_id: u32, msg: &str) {
        if !self.map.contains_key(&msg_bnd) {
            self.map.insert(msg_bnd, HashMap::new());
        }
        if !self.map[&msg_bnd].contains_key(&msg_id) {
            let h = HSTRING::from(msg);
            let pcw: PCWSTR = PCWSTR(h.as_ptr());
            let str_box = Box::new(pcw);
            if let Some(bnd) = self.map.get_mut(&msg_bnd) {
                bnd.insert(msg_id, str_box);
            }
        }
    }
}

pub fn init_message() -> MessageData{
    let map:HashMap<u32, HashMap<u32, Box<PCWSTR>>> = HashMap::new();

    let hook_handle_message = make_installer_from_call_aob::<GetMessageType>(GET_MESSAGE_AOB, GET_MESSAGE_OFFSET, &GET_MESSAGE_ORIGINAL_HOLDER).unwrap()
    .install_mut({
        move |original| move |message_repository, _1, msg_bnd, msg_id| {
            if msg_bnd == 10 {
                if msg_id == 67350 {
                    return w!("Modify Intensity By:");
                }
                if msg_id == 67351 {
                    return w!("Grace Ascetic");
                }
            }
            if msg_bnd == 20 {
                if msg_id == 67351 {
                    return w!("Grace Ascetic Info");
                }
            }
            if msg_bnd == 24 {
                if msg_id == 67351 {
                    return w!("Grace Ascetic Caption");
                }
            }
            if msg_bnd == 33 {
                if msg_id == 22021100 {
                    return w!("Increase Intensity (Current: <?loopCount?>)");
                }
                if msg_id == 22021101 {
                    return w!("Decrease Intensity (Current: <?loopCount?>)");
                }
                if msg_id == 22021102 {
                    return w!("Current Intensity: <?loopCount?>");
                }
                if msg_id == 22021103 {
                    return w!("Intensity Updated");
                }
            }
            return unsafe { original(message_repository, _1, msg_bnd, msg_id) };
        }
    }).unwrap();

    unsafe {
        hook_handle_message.enable(true);
    };

    return MessageData{map: map, hook:hook_handle_message};
}