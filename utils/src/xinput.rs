use std::collections::{HashMap, hash_map::Entry};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::UI::Input::XboxController::*;

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(250);

type DebounceKey = (u32, u16); // (controller_index, button)
type DebounceMap = HashMap<DebounceKey, Instant>;

static DEBOUNCE_MAP: LazyLock<Mutex<DebounceMap>> =
    LazyLock::new(Default::default);

pub fn is_button_pressed(controller: u32, button: XINPUT_GAMEPAD_BUTTON_FLAGS) -> bool {
    unsafe {
        let mut state = XINPUT_STATE::default();

        unsafe {
            if XInputGetState(controller, &mut state) == ERROR_SUCCESS.0 {
                let pressed = state.Gamepad.wButtons.contains(button);

                if pressed {
                    let now = Instant::now();
                    let key = (controller, button.0);

                    match DEBOUNCE_MAP.lock().unwrap().entry(key) {
                        Entry::Occupied(mut o) => {
                            if o.get().elapsed() > DEBOUNCE_TIMEOUT {
                                o.insert(now);
                                return true;
                            }
                            return false;
                        }
                        Entry::Vacant(v) => {
                            v.insert(now);
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}