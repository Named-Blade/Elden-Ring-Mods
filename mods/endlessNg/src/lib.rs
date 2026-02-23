use std::time::Duration;
use std::{thread, time};
use std::mem::MaybeUninit;

use eldenring::{
    cs::{
        BlockId, CSTaskGroupIndex, CSTaskImp, GameDataMan, WorldChrMan, SoloParam, SoloParamRepository, ClearCountCorrectParam,
        EzStateInvokeError, FieldInsHandle, FieldInsSelector, TalkScript,
    },
    fd4::FD4TaskData,
    param::CLEAR_COUNT_CORRECT_PARAM_ST,
    ez_state::EzStateValue,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{FromStatic, program::Program, task::*};

mod console;
use console::init_console;

mod log;
use log::*;

mod patch;
use patch::perform_patch;

mod clear;
use clear::{ClearCountField, ClearCountFieldAccess};

mod talk;
use talk::*;

mod hook;

mod msg;
use msg::*;

mod goods;
use goods::*;

use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY, VK_T, VK_Y};
fn is_key_down(key: VIRTUAL_KEY) -> bool {
    let key_state = unsafe { GetKeyState(key.0 as i32) } as u16;
    key_state & 0x8000 != 0
}

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
            if let Some(row) =
                unsafe { repo.get_row_by_index_mut::<ClearCountCorrectParam>(row_index) }
            {
                //the typo is how it's actually named
                row.set_netural_attack_rate(1.0);
            }
    }
}

#[derive(Default, Debug)]
enum IntensityState {
    #[default]
    Idle,
    Enter,
    WaitForDialog,
    DisplayIntensityDialogue,
    WaitForGenericDialog,
    Done,
}

struct Intensity {
    change_sign: i32, // FlagState, default ON
}

impl StateMachine for Intensity {
    type State = IntensityState;

    fn step_state(
        &mut self,
        state: IntensityState,
        ts: &mut TalkScript,
    ) -> Result<Transition<IntensityState>, EzStateInvokeError> {
        use Transition::*;

        // Shorthand helpers
        macro_rules! env {
            ($cmd:expr) => { ts.env($cmd)? };
        }
        macro_rules! event {
            ($cmd:expr) => { ts.event($cmd)?; };
        }
        macro_rules! i {
            ($v:expr) => { EzStateValue::Int32($v) };
        }

        Ok(match state {
            IntensityState::Idle => {
                let has_item: i32 = env!((DOES_PLAYER_HAVE_ITEM, [i!(ITEM_TYPE_GOODS), i!(67351)])).into();
                if has_item == 0 {
                    event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(67351), i!(1)]));
                }
                if is_key_down(VK_T) {
                    self.change_sign = 0;
                    Next(IntensityState::Enter) 
                } else if is_key_down(VK_Y) {
                    self.change_sign = 1;
                    Next(IntensityState::Enter) 
                } else { 
                    Transition::<IntensityState>::Wait(IntensityState::Idle) 
                }
            }

            IntensityState::Enter => {
                let limit: i32 = env!((GET_ITEM_HELD_NUM_LIMIT, [i!(ITEM_TYPE_GOODS), i!(67350)])).into();

                event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(67350), i!(-limit)]));
                event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(67350), i!(limit)]));
                event!(CLEAR_QUANTITY_VALUE_OF_CHOOSE_QUANTITY_DIALOG);
                event!((OPEN_CHOOSE_QUANTITY_DIALOG, [i!(67350), i!(22021102)]));

                Next(IntensityState::WaitForDialog)
            }

            IntensityState::WaitForDialog => {
                let menu_open: i32  = env!((CHECK_SPECIFIC_PERSON_MENU_IS_OPEN,          [i!(13), i!(0)])).into();
                let dialog_open: i32 = env!((CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN, [i!(0)])).into();

                if menu_open == 1 && dialog_open == 0 {
                    return Ok(Transition::<IntensityState>::Wait(IntensityState::WaitForDialog)); // still waiting; don't advance state
                }

                let value: i32 = env!(GET_VALUE_FROM_NUMBER_SELECT_DIALOG).into();
                if value >= 0 {
                    let Ok(game_data_man) = (unsafe { GameDataMan::instance() }) else {
                        let limit: i32 = env!((GET_ITEM_HELD_NUM_LIMIT, [i!(ITEM_TYPE_GOODS), i!(67350)])).into();
                        event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(67350), i!(-limit)]));
                        return Ok(Transition::<IntensityState>::Done);
                    };
                    if self.change_sign == 0 {
                        game_data_man.ng_lvl = game_data_man.ng_lvl.saturating_sub(value as u32);
                    }  else {
                        game_data_man.ng_lvl = game_data_man.ng_lvl.saturating_add(value as u32);
                    }
                    log!("set ng level to {}", game_data_man.ng_lvl);
                }

                let limit: i32 = env!((GET_ITEM_HELD_NUM_LIMIT, [i!(ITEM_TYPE_GOODS), i!(67350)])).into();
                event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(67350), i!(-limit)]));
                Next(IntensityState::DisplayIntensityDialogue)
            }

            IntensityState::DisplayIntensityDialogue => {
                event!((OPEN_GENERIC_DIALOG, [
                    i!(DIALOG_BOX_TYPE_CENTER_BOTTOM_1), i!(22021103),
                    i!(DIALOG_RESULT_LEFT), i!(DIALOG_BOX_STYLE_ORNATE_NO_OPTIONS), i!(1),
                ]));
                Next(IntensityState::WaitForGenericDialog)
            }

            IntensityState::WaitForGenericDialog => {
                let dialog_open: i32 = env!((CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN, [i!(0)])).into();
                if dialog_open == 1 { return Ok(Transition::<IntensityState>::Wait(IntensityState::WaitForGenericDialog)); }
                Next(IntensityState::Done)
            }

            IntensityState::Done => Transition::<IntensityState>::Done,
        })
    }
}

unsafe impl Send for StateRunner<Intensity> {}

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
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        thread::sleep(time::Duration::from_secs(10));

        let mut goods_data = init_goods();

        goods_data.add_instance(
            67350,
            2912,
            vec![(EquipParamGoodsField::MaxNum, 9999.0)]
        );
        goods_data.add_instance(
            67351,
            2912,
            vec![
                (EquipParamGoodsField::MaxNum, 1.0),
                (EquipParamGoodsField::MaxRepositoryNum, 0.0),
                (EquipParamGoodsField::IsDrop, 0.0),
                (EquipParamGoodsField::IsDiscard, 0.0),
                (EquipParamGoodsField::IsConsume, 0.0),
                (EquipParamGoodsField::IsDeposit, 0.0),
                (EquipParamGoodsField::Rarity, 5.0),
                (EquipParamGoodsField::SellValue, -1.0),
                (EquipParamGoodsField::SortId, 349.0),
                (EquipParamGoodsField::SortGroupId, 10.0),
                (EquipParamGoodsField::IconId, 9.0)
            ]
        );

        let mut message_data = init_message();
        message_data.add_message(BND_GOODS_NAME, 67350, "Modify Intensity By:");
        message_data.add_message(BND_GOODS_NAME, 67351, "Grace Ascetic");
        message_data.add_message(BND_GOODS_INFO, 67351, "Grace Ascetic Info");
        message_data.add_message(BND_GOODS_CAPTION, 67351, "Grace Ascetic Caption");
        message_data.add_message(BND_TALK, 22021100, "Increase Intensity (Current: <?loopCount?>)");
        message_data.add_message(BND_TALK, 22021101, "Decrease Intensity (Current: <?loopCount?>)");
        message_data.add_message(BND_TALK, 22021102, "Current Intensity: <?loopCount?>");
        message_data.add_message(BND_TALK, 22021103, "Intensity Updated to <?loopCount?>");

        //remove health cap
        let health_cap_aob = "eb 14 81 fa ff ff 07 00 48 8d 44 24 18 4c 8d 44 24 10 49 0f 4e c0 8b 10 89 91 3c 01 00 00";
        let health_cap_offset = 18;
        let health_cap_expected = "49 0f 4e c0";// cmovle rax,r8;
        let health_cap_patch = "49 8b c0 90";   // mov rax,r8; nop;
        perform_patch(health_cap_aob,health_cap_expected,health_cap_patch,health_cap_offset);

        let Ok(solo_param_repository) = (unsafe { SoloParamRepository::instance() }) else { return; };

        fix_attack_rate(solo_param_repository);
        let (cycle_increase, original_max) = compute_clear_count_cycle_increase(solo_param_repository);

        let mut runner = Box::new(StateRunner::new(
            Intensity { change_sign: 1 },
            TalkScript::new(
                BlockId::none(),
                1000,
                FieldInsHandle { block_id: BlockId::none(), selector: FieldInsSelector(0) },
            ),
        ));

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

                if let Ok(world_chr_man) = unsafe { WorldChrMan::instance() }
                && let Some(ref mut main_player) = world_chr_man.main_player
                {
                    runner.talk_script.npc_talk.base.field_ins_handle =
                        main_player.chr_ins.field_ins_handle;

                    if let Err(e) = runner.step() {
                        log!("{:?}", e);
                        runner.state = IntensityState::Idle;
                    }
                }

            },
            CSTaskGroupIndex::FrameBegin,
        );

        thread::park();
    });

    true
}