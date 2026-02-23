use std::sync::{Arc, OnceLock};
use std::collections::HashMap;
use windows::core::PCWSTR;
use winhook::HookHandle;

use crate::hook::*;

pub const BND_GOODS_NAME: u32 = 10;
pub const BND_GOODS_INFO: u32 = 20;
pub const BND_GOODS_CAPTION: u32 = 24;
pub const BND_TALK: u32 = 33;

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
    map: HashMap<u32, HashMap<u32, Box<[u16]>>>,
    hook: Option<HookHandle>
}

impl MessageData {
    pub fn add_message(&mut self, msg_bnd: u32, msg_id: u32, msg: &str) {
        let entry = self.map.entry(msg_bnd).or_insert_with(HashMap::new);

        if !entry.contains_key(&msg_id) {
            // Convert to UTF-16 and append null terminator
            let mut wide: Vec<u16> = msg.encode_utf16().collect();
            wide.push(0);

            entry.insert(msg_id, wide.into_boxed_slice());
        }
    }

    pub fn get_message(&self, msg_bnd: u32, msg_id: u32) -> Option<PCWSTR> {
        self.map
            .get(&msg_bnd)?
            .get(&msg_id)
            .map(|buf| PCWSTR(buf.as_ptr()))
    }

    pub fn set_hook(&mut self, hook: HookHandle) {
        self.hook = Some(hook);
    }
}

pub struct MessageContainer {
    arc: Arc<MessageData>
}

impl MessageContainer {
    pub fn add_message(&mut self, msg_bnd: u32, msg_id: u32, msg: &str) {
        unsafe {
            let ptr = Arc::as_ptr(&self.arc) as *mut MessageData;
            (*ptr).add_message(msg_bnd, msg_id, msg);
        }
    }
}

pub fn init_message() -> MessageContainer {
    // Shared, immutable pointer for closure
    let message_data = Arc::new(MessageData {
        map: HashMap::new(),
        hook: None,
    });

    // Clone for closure use
    let closure_data = Arc::clone(&message_data);

    let hook_handle_message = make_installer_from_call_aob::<GetMessageType>(
        GET_MESSAGE_AOB,
        GET_MESSAGE_OFFSET,
        &GET_MESSAGE_ORIGINAL_HOLDER,
    )
    .unwrap()
    .install_mut({
        move |original| {
            let closure_data = Arc::clone(&closure_data);
            move |message_repository, _1, msg_bnd, msg_id| {
                if let Some(msg) = closure_data.get_message(msg_bnd, msg_id) {
                    return msg;
                } else {
                    unsafe { original(message_repository, _1, msg_bnd, msg_id) }
                }
            }
        }
    })
    .unwrap();

    unsafe { hook_handle_message.enable(true) };
    //this is stupid
    let raw = Arc::into_raw(message_data) as *mut MessageData;
    unsafe {
        (*raw).set_hook(hook_handle_message);
        return MessageContainer{arc: Arc::from_raw(raw)};
    }
}