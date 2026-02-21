use std::time::Duration;
use std::{thread, time};
use std::mem::MaybeUninit;

mod console;
use console::init_console;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, GameDataMan, SoloParamRepository, ClearCountCorrectParam},
    fd4::FD4TaskData,
    param::CLEAR_COUNT_CORRECT_PARAM_ST,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{FromStatic, program::Program, task::*};

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

        let Ok(solo_param_repository) = (unsafe { SoloParamRepository::instance() }) else {
            return;
        };

        let (cycle_increase, original_max) = compute_clear_count_cycle_increase(solo_param_repository);

        // Retrieve games task runner and register a task at frame begin.
        let cs_task = unsafe { CSTaskImp::instance().unwrap() };
        cs_task.run_recurring(
            move |_: &FD4TaskData| { 
                let Ok(game_data_man) = (unsafe { GameDataMan::instance() }) else {
                    return;
                };

                let Ok(repo) = (unsafe { SoloParamRepository::instance() }) else {
                    return;
                };

                eprintln!("{:p}", &game_data_man.ng_lvl);
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