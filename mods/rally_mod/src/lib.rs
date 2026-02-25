use std::time::Duration;
use std::{thread, time};
use std::sync::OnceLock;
use std::ptr;
use eldenring::{
    cs::GameMan,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{program::Program,FromStatic};

use utils::*;

use console::*;
use log::*;
use hook::*;

const RALLY_UPDATE_AOB: &str = "48 8b 09 e8 ? ? ? ? 48 8b 87 90 01 00 00 48 8b 08 e8";
const RALLY_HUPDATE_OFFSET: usize = 4;

#[repr(C, packed)]
pub struct RallyData {
    pub rally_potential: f32,
    pub rally_cap: f32,
    pub rally_timer: f32,
    pub rally_regain: f32,
}

#[repr(C, packed)]
pub struct CSChrDataModule {
    pub _1: [u8; 0x138],
    pub current_hp: i32,
    pub max_hp: i32,
    pub _2: [u8; 0x20],
    pub rally_data: RallyData,
}

pub type RallyUpdateType = unsafe extern "C" fn(
    chr_data: *mut CSChrDataModule,
    delta_time: f32,
);

pub static RALLY_UPDATE_ORIGINAL_HOLDER: OnceLock<RallyUpdateType> = OnceLock::new();

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: isize, reason: u32) -> bool {
    // Exit early if we're not attaching a DLL
    if reason != 1 {
        return true;
    }

    unsafe{set_dll_hmodule(hmodule)};
    init_console();
    log!("mod started");

    std::thread::spawn(move || {

        let _ = config::init(config::Schema::new()
            .field("rally_mod", "rally", true, None::<String>)
            .field("loading", "wait_time", 10_i64, None::<String>)
        );

        let wait_time = config::get_int("loading", "wait_time").unwrap() as u64;

        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        thread::sleep(time::Duration::from_secs(wait_time));

        let hook_rally = make_installer_from_call_aob::<RallyUpdateType>(
            RALLY_UPDATE_AOB,
            RALLY_HUPDATE_OFFSET,
            &RALLY_UPDATE_ORIGINAL_HOLDER,
        )
        .unwrap()
        .install_mut({
            move |_original| {
                move |chr_data, delta_time| {
                    if let Ok(game_man) = unsafe {GameMan::instance()} {
                        let is_rally_disabled = unsafe {*((game_man as *mut GameMan as usize + 0xdb7) as *mut bool)};
                        if !is_rally_disabled {
                            let (
                                mut current_hp,
                                max_hp,
                                mut rally_potential,
                                mut rally_cap,
                                mut rally_timer,
                                mut rally_regain,
                            ) = unsafe {
                                let aligned: CSChrDataModule = ptr::read_unaligned(chr_data);
                                (
                                    aligned.current_hp,
                                    aligned.max_hp,
                                    aligned.rally_data.rally_potential,
                                    aligned.rally_data.rally_cap,
                                    aligned.rally_data.rally_timer,
                                    aligned.rally_data.rally_regain,
                                )
                            };

                            if rally_timer < delta_time {
                                rally_timer = 0.0;
                                rally_cap = 0.0;
                            } else {
                                rally_timer -= delta_time;
                            }

                            if rally_potential > rally_cap {
                                let change = max_hp as f32 * 1.0 * delta_time;
                                if (rally_potential - change) < rally_cap {
                                    rally_potential = rally_cap;
                                } else {
                                    rally_potential -= change;
                                }
                            }

                            let mut to_regain: f32 = 0.0;
                            let hp_to_full = (max_hp - current_hp) as f32;
                            if rally_regain > 1.0 && current_hp > 0 {
                                to_regain = rally_regain;
                                if rally_potential < to_regain {
                                    to_regain = rally_potential;
                                }
                                if hp_to_full < to_regain {
                                    to_regain = hp_to_full;
                                }
                            }

                            current_hp += to_regain.ceil() as i32;
                            rally_potential -= to_regain;
                            rally_cap -= to_regain;
                            rally_regain = 0.0;

                            unsafe {
                                (*chr_data).current_hp = current_hp;
                                (*chr_data).rally_data.rally_potential = rally_potential;
                                (*chr_data).rally_data.rally_cap = rally_cap;
                                (*chr_data).rally_data.rally_timer = rally_timer;
                                (*chr_data).rally_data.rally_regain = rally_regain;
                            }
                        }
                    }
                }
            }
        })
        .unwrap();

        unsafe { hook_rally.enable(true) };

        thread::park();
    });

    true
}