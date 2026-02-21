use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, GameDataMan, SoloParam, SoloParamRepository, ClearCountCorrectParam},
    param::CLEAR_COUNT_CORRECT_PARAM_ST,
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{FromStatic, program::Program, task::*};

struct ClearCountCorrectParamIter {
    row_index: usize,
    row_count: usize,
}

impl Iterator for ClearCountCorrectParamIter {
    type Item = usize; // row index

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index < self.row_count {
            let idx = self.row_index;
            self.row_index += 1;
            Some(idx)
        } else {
            None
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

    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        // Retrieve games task runner and register a task at frame begin.
        let cs_task = unsafe { CSTaskImp::instance().unwrap() };
        cs_task.run_recurring(
            |_: &FD4TaskData| {
                // Retrieve GameDataMan
                let Ok(game_data_man) = (unsafe { GameDataMan::instance() }) else {
                    return;
                };

                let Ok(solo_param_repository) = (unsafe { SoloParamRepository::instance() }) else {
                    return;
                };

                let holder = &solo_param_repository.solo_param_holders[ClearCountCorrectParam::INDEX as usize];
                let res_cap = match holder.get_res_cap(0) { Some(r) => r, None => return, };
                let param_file = unsafe { res_cap.param_res_cap.as_ref().data.as_ref() };
                let row_count = param_file.row_count();
                let iter = ClearCountCorrectParamIter { row_index: 0, row_count };
                for row_idx in iter {
                    unsafe {
                        if let Some(row) = solo_param_repository.get_row_by_index_mut::<ClearCountCorrectParam>(row_idx) {
                            row.set_soul_rate(100.0);
                        }
                    }
                }


            },
            CSTaskGroupIndex::FrameBegin,
        );
    });

    true
}