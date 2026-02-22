use std::time::Duration;
use std::{thread, time};
use std::mem::MaybeUninit;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, GameDataMan, SoloParam, SoloParamRepository, ClearCountCorrectParam},
    fd4::FD4TaskData,
    param::CLEAR_COUNT_CORRECT_PARAM_ST,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{FromStatic, program::Program, task::*};

mod console;
use console::init_console;

mod patch;
use patch::perform_patch;

mod clear;
use clear::{ClearCountField, ClearCountFieldAccess};

fn compute_clear_count_cycle_increase(
    repo: &mut SoloParamRepository,
) -> (CLEAR_COUNT_CORRECT_PARAM_ST, CLEAR_COUNT_CORRECT_PARAM_ST) {
    let mut cycle_increase: CLEAR_COUNT_CORRECT_PARAM_ST = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut original_max: CLEAR_COUNT_CORRECT_PARAM_ST = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut cycle_increase_counter: CLEAR_COUNT_CORRECT_PARAM_ST = unsafe { MaybeUninit::zeroed().assume_init() };

    // There are at least 8 rows assumed in the param file
    for i in 2..8 {
        // Get row i-1 and i
        let row1_index = repo
            .get_index_by_param_id::<ClearCountCorrectParam>(i - 1)
            .expect("Row i-1 missing");

        let row2_index = repo
            .get_index_by_param_id::<ClearCountCorrectParam>(i)
            .expect("Row i missing");

        // SAFETY: row1_index != row2_index, so these mutable references do not alias
        let row1: &mut CLEAR_COUNT_CORRECT_PARAM_ST = unsafe {
            let ptr = repo.get_row_by_index_mut::<ClearCountCorrectParam>(row1_index).unwrap() as *mut _;
            &mut *ptr
        };

        let row2: &mut CLEAR_COUNT_CORRECT_PARAM_ST = unsafe {
            let ptr = repo.get_row_by_index_mut::<ClearCountCorrectParam>(row2_index).unwrap() as *mut _;
            &mut *ptr
        };

        // Increment the cycleIncreaseCounter for each field
        for &field in ClearCountField::ALL.iter() {
            let val = cycle_increase_counter.get_field(field)
                + row2.get_field(field)
                - row1.get_field(field);
            cycle_increase_counter.set_field(field, val);
        }

        // On the last iteration, save original max
        if i == 7 {
            for &field in ClearCountField::ALL.iter() {
                original_max.set_field(field, row2.get_field(field));
            }
        }
    }

    // Compute average increase per NG cycle
    for &field in ClearCountField::ALL.iter() {
        let val = (cycle_increase_counter.get_field(field)) / 6.0;
        cycle_increase.set_field(field, val);
    }

    (cycle_increase, original_max)
}

//vanilla accidentaly increases physical damage on ng cycles too much by applying it multiplicatively on both the physical damage supertype and all physical damage subtypes
fn fix_attack_rate(repo: &mut SoloParamRepository) {
    let holder = &repo.solo_param_holders[ClearCountCorrectParam::INDEX as usize];
    let res_cap = match holder.get_res_cap(0) {
        Some(rc) => rc,
        None => return,
    };
    let param_res = unsafe { res_cap.param_res_cap.as_ref() };
    let data = &param_res.data;
    let row_count = data.row_count();
        for row_index in 0..row_count {
        unsafe {
            if let Some(row) =
                repo.get_row_by_index_mut::<ClearCountCorrectParam>(row_index)
            {
                //the typo is how it's actually named
                row.set_netural_attack_rate(1.0);
            }
        }
    }
}

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(_hmodule: u64, reason: u32) -> bool {
    // Exit early if we're not attaching a DLL
    if reason != 1 {
        return true;
    }
    init_console();

    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        thread::sleep(time::Duration::from_secs(10));

        //remove health cap
        let health_cap_aob = "eb 14 81 fa ff ff 07 00 48 8d 44 24 18 4c 8d 44 24 10 49 0f 4e c0 8b 10 89 91 3c 01 00 00";
        let health_cap_offset = 18;
        let health_cap_expected = "49 0f 4e c0";// cmovle rax,r8;
        let health_cap_patch = "49 8b c0 90";   // mov rax,r8; nop;
        perform_patch(health_cap_aob,health_cap_expected,health_cap_patch,health_cap_offset);

        let Ok(solo_param_repository) = (unsafe { SoloParamRepository::instance() }) else { return; };

        fix_attack_rate(solo_param_repository);
        let (cycle_increase, original_max) = compute_clear_count_cycle_increase(solo_param_repository);

        // Retrieve games task runner and register a task at frame begin.
        let cs_task = unsafe { CSTaskImp::instance().unwrap() };
        cs_task.run_recurring(
            move |_: &FD4TaskData| { 
                let Ok(game_data_man) = (unsafe { GameDataMan::instance() }) else { return; };
                let Ok(repo) = (unsafe { SoloParamRepository::instance() }) else { return; };

                if game_data_man.ng_lvl > 6 {
                    let row1_index = repo
                        .get_index_by_param_id::<ClearCountCorrectParam>(7)
                        .expect("Row 7 missing");
                    let row2_index = repo
                        .get_index_by_param_id::<ClearCountCorrectParam>(107)
                        .expect("Row 107 missing");
                    let row1: &mut CLEAR_COUNT_CORRECT_PARAM_ST = unsafe {
                        let ptr = repo.get_row_by_index_mut::<ClearCountCorrectParam>(row1_index).unwrap() as *mut _;
                        &mut *ptr
                    };
                    let row2: &mut CLEAR_COUNT_CORRECT_PARAM_ST = unsafe {
                        let ptr = repo.get_row_by_index_mut::<ClearCountCorrectParam>(row2_index).unwrap() as *mut _;
                        &mut *ptr
                    };
                    for &field in ClearCountField::ALL.iter() {
                        if field != ClearCountField::SuperArmorDamageRate {
                            row1.set_field(field, original_max.get_field(field) + cycle_increase.get_field(field) * (game_data_man.ng_lvl - 7) as f32);
                            row2.set_field(field, original_max.get_field(field) + cycle_increase.get_field(field) * (game_data_man.ng_lvl - 7) as f32);
                        }
                    }
                }

            },
            CSTaskGroupIndex::FrameBegin,
        );
    });

    true
}