use std::time::Duration;
use std::{thread, time};
use std::sync::OnceLock;
use std::f64::consts;
use std::ptr;
use pelite::pattern::parse;
use eldenring::{
    cs::{GameMan, WorldChrMan},
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{program::Program,FromStatic};

use utils::*;

use console::*;
use log::*;
use patch::*;
use hook::*;

const RALLY_UPDATE_AOB: &str = "48 8b 09 e8 ? ? ? ? 48 8b 87 90 01 00 00 48 8b 08 e8";
const RALLY_HUPDATE_OFFSET: usize = 4;

const RALLY_HP_CHANGE_AOB: &str = "C6 44 24 28 01 33 D2 F3 0F 11 44 24 20 48 8B 09 E8 ? ? ? ? 48 8B 4B 58 33 D2 E8";
const RALLY_HP_CHANGE_OFFSET: usize = 17;

const GLOBAL_SOUND_AOB: &str = "ba b4 00 00 00 48 8d 0d ? ? ? ? e8 ? ? ? ? 48 8b 0d ? ? ? ? e8 ? ? ? ? 84 c0 0f 94 c2 eb 02 32 d2 f6 c3 01 74 07 83 e3 fe";
const GLOBAL_SOUND_OFFSET: usize = 20;

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

type RallyModifyType = unsafe extern "C" fn(
    chr_data: *mut CSChrDataModule,
    new_hp: i32,
    reset_rally_flag: u8,
    rally_gain_multiplier: f32,
    rally_time_multiplier: f32,
    force_timer_refresh_flag: u8,
);

pub static RALLY_UPDATE_ORIGINAL_HOLDER: OnceLock<RallyUpdateType> = OnceLock::new();
pub static RALLY_HP_CHANGE_ORIGINAL_HOLDER: OnceLock<RallyModifyType> = OnceLock::new();

fn is_rally_disabled(game_man: *mut GameMan) -> bool {
    return unsafe {*((game_man as *mut GameMan as usize + 0xdb7) as *mut bool)};
}

fn is_main_player(world_chr_man: *const WorldChrMan, chr_data: *const CSChrDataModule) -> bool {
    let Some(main_player)  = (unsafe { (*world_chr_man).main_player.as_ref() }) else {return false;};
    let data: *const eldenring::cs::CSChrDataModule = chr_data as *const eldenring::cs::CSChrDataModule;
    return main_player.as_ptr() as *mut () == unsafe{(*data).owner.as_ptr() as *mut ()};
}

fn is_in_combat(sound_global: Option<usize>) -> bool {
    match sound_global {
        Some(global) => {
            unsafe {
                let global = global as *const usize;
                if global.is_null() { return false; };
                let global = *global;
                let bgm_controller: *const () = *((global as usize + 0x328) as *const *const ());
                if bgm_controller.is_null() { return false; };
                let is_in_combat: bool = *((bgm_controller as usize + 0x4) as *const bool);
                is_in_combat
            }
        }
        None => {false}
    }
}

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
            .field("rally_mod", "rally_time", 4_f64, None::<String>)
            .field("rally_mod", "rally_hit_reset", true, None::<String>)
            .field("rally_mod", "rally_only_heal", true, None::<String>)
            .field("rally_mod", "exponential_decay", true, None::<String>)
            .field("rally_mod", "half_life", 7.5_f64, None::<String>)
            .field("rally_mod", "rally_decay", 15_f64, None::<String>)
            .field("loading", "wait_time", 10_i64, None::<String>)
        );

        let wait_time = config::get_int("loading", "wait_time").unwrap() as u64;

        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        thread::sleep(time::Duration::from_secs(wait_time));

        let sound_global: Option<usize> = {
            let aob = parse(GLOBAL_SOUND_AOB).unwrap();
            if let Some(address) = aob_scan(&aob) {
                let rip = address as usize + GLOBAL_SOUND_OFFSET + 4;
                let address = address as usize + GLOBAL_SOUND_OFFSET;
                let value = unsafe {std::ptr::read_unaligned(address as *const u32)} as usize;
                log!("{:p}", (value+rip) as *const ());
                Some(value + rip)
            } else {
                None
            }
        };

        let rally_time = config::get_float("rally_mod", "rally_time").unwrap() as f32;
        let rally_hit_reset = config::get_bool("rally_mod", "rally_hit_reset").unwrap();
        let rally_only_heal = config::get_bool("rally_mod", "rally_hit_reset").unwrap();
        
        let exponential_decay = config::get_bool("rally_mod", "exponential_decay").unwrap();
        let half_life = config::get_float("rally_mod", "half_life").unwrap() as f32;

        let rally_decay = config::get_float("rally_mod", "rally_decay").unwrap() as f32;

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
                        if !is_rally_disabled(game_man) {
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
                                rally_cap = 0.0;
                            }
                            rally_timer -= delta_time;
                            

                            if rally_potential >= rally_cap {
                                let change = {
                                    if exponential_decay {
                                        let k: f32 = 2_f32.ln() / half_life;
                                        let beyond_cap = rally_potential - rally_cap;
                                        beyond_cap - (beyond_cap * (consts::E as f32).powf(-k * delta_time))
                                    } else {
                                        let decay = 1.0/rally_decay;
                                        max_hp as f32 * decay * delta_time 
                                    }
                                };
                                if (rally_potential - change) < rally_cap {
                                    rally_potential = rally_cap;
                                } else {
                                    rally_potential -= change;
                                }
                            }

                            let hp_to_full = (max_hp - current_hp) as f32;
                            if rally_potential > (hp_to_full) {
                                rally_potential = hp_to_full;
                            }

                            let mut to_regain: f32 = 0.0;
                            if rally_regain > 1.0 && current_hp > 0 {
                                to_regain = rally_regain;
                                if rally_potential < to_regain {
                                    to_regain = rally_potential;
                                }
                                if hp_to_full < to_regain {
                                    to_regain = hp_to_full;
                                }
                            }

                            if rally_hit_reset && rally_regain > 0.0 {
                                rally_timer = rally_time;
                                rally_cap = rally_potential
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

        let hook_rally_change = make_installer_from_call_aob::<RallyModifyType>(
            RALLY_HP_CHANGE_AOB,
            RALLY_HP_CHANGE_OFFSET,
            &RALLY_HP_CHANGE_ORIGINAL_HOLDER,
        )
        .unwrap()
        .install_mut({
            move |_original| {
               move |chr_data,
                    new_hp: i32,
                    reset_rally_flag: u8,
                    rally_gain_multiplier: f32,
                    rally_time_multiplier: f32,
                    force_timer_refresh_flag: u8| {

                    let reset_rally = reset_rally_flag != 0;
                    let force_timer_refresh = force_timer_refresh_flag != 0;

                    unsafe {
                        let data = &mut *chr_data;

                        let old_hp = data.current_hp;
                        let max_hp = data.max_hp;

                        // --- 1. Clamp and assign HP ---
                        let clamped_hp = new_hp.clamp(0, max_hp);
                        data.current_hp = clamped_hp;

                        let hp_delta = clamped_hp - old_hp;

                        // --- 2. Rally only applies if special effect active ---
                        let Ok(game_man) = GameMan::instance() else { return; };
                        let Ok(world_chr_man) = WorldChrMan::instance() else { return; };
                        if !is_rally_disabled(game_man)
                            && is_main_player(world_chr_man, chr_data)
                            && true//has_special_effect_449(data)
                        {
                            let rally = &mut data.rally_data;

                            // --- 3. Hard reset ---
                            if reset_rally {
                                rally.rally_cap = 0.0;
                                rally.rally_timer = 0.0;
                                rally.rally_regain = 0.0;
                            }

                            let missing_hp = (max_hp - clamped_hp) as f32;

                            // --- 4A. Damage taken and rally gain allowed ---
                            if hp_delta < 0 && rally_gain_multiplier > 0.0 {
                                let damage_taken = (-hp_delta) as f32;
                                let rally_gain = damage_taken * rally_gain_multiplier;

                                rally.rally_potential += rally_gain;
                                rally.rally_cap += rally_gain;

                                // Clamp potential to missing HP
                                rally.rally_potential =
                                    rally.rally_potential.clamp(0.0, missing_hp);

                                // Cap cannot exceed potential
                                rally.rally_cap =
                                    rally.rally_cap.clamp(0.0, rally.rally_potential);

                                // Regain cannot exceed potential
                                rally.rally_regain =
                                    rally.rally_regain.clamp(0.0, rally.rally_potential);

                                // Refresh timer
                                if force_timer_refresh || rally.rally_timer <= 0.0 {
                                    rally.rally_timer = rally_time * rally_time_multiplier;
                                }
                            }
                            // --- 4B. No damage / healing branch ---
                            else {
                                rally.rally_potential =
                                    rally.rally_potential.clamp(0.0, missing_hp);

                                rally.rally_cap =
                                    rally.rally_cap.clamp(0.0, rally.rally_potential);

                                rally.rally_regain =
                                    rally.rally_regain.clamp(0.0, rally.rally_potential);
                            }

                            if rally_only_heal && hp_delta > 0 && is_in_combat(sound_global) {
                                rally.rally_potential += hp_delta as f32;
                                rally.rally_cap += hp_delta as f32;
                                data.current_hp -= hp_delta - 1;

                                let new_timer = hp_delta as f32 /200.0;
                                if rally.rally_timer < new_timer {
                                    rally.rally_timer = new_timer;
                                }
                            }
                        }
                    }
                }
            }
        })
        .unwrap();

        unsafe { 
            hook_rally.enable(true);
            hook_rally_change.enable(true);
        };

        thread::park();
    });

    true
}