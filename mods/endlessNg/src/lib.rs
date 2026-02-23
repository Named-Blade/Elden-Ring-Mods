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

mod config;

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
fn fix_physical_damage_scaling(repo: &mut SoloParamRepository) {
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
    goods_display_id: i32,
    goods_intensity_id: i32,
    current_talk_id: i32,
    update_talk_id: i32,
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
                let has_item: i32 = env!((DOES_PLAYER_HAVE_ITEM, [i!(ITEM_TYPE_GOODS), i!(self.goods_intensity_id)])).into();
                if has_item == 0 {
                    event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(self.goods_intensity_id), i!(1)]));
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
                let limit: i32 = env!((GET_ITEM_HELD_NUM_LIMIT, [i!(ITEM_TYPE_GOODS), i!(self.goods_display_id)])).into();

                event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(self.goods_display_id), i!(-limit)]));
                event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(self.goods_display_id), i!(limit)]));
                event!(CLEAR_QUANTITY_VALUE_OF_CHOOSE_QUANTITY_DIALOG);
                event!((OPEN_CHOOSE_QUANTITY_DIALOG, [i!(self.goods_display_id), i!(self.current_talk_id)]));

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
                        let limit: i32 = env!((GET_ITEM_HELD_NUM_LIMIT, [i!(ITEM_TYPE_GOODS), i!(self.goods_display_id)])).into();
                        event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(self.goods_display_id), i!(-limit)]));
                        return Ok(Transition::<IntensityState>::Done);
                    };
                    if self.change_sign == 0 {
                        game_data_man.ng_lvl = game_data_man.ng_lvl.saturating_sub(value as u32);
                    }  else {
                        game_data_man.ng_lvl = game_data_man.ng_lvl.saturating_add(value as u32);
                    }
                    log!("set ng level to {}", game_data_man.ng_lvl);
                }

                let limit: i32 = env!((GET_ITEM_HELD_NUM_LIMIT, [i!(ITEM_TYPE_GOODS), i!(self.goods_display_id)])).into();
                event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(self.goods_display_id), i!(-limit)]));
                Next(IntensityState::DisplayIntensityDialogue)
            }

            IntensityState::DisplayIntensityDialogue => {
                event!((OPEN_GENERIC_DIALOG, [
                    i!(DIALOG_BOX_TYPE_CENTER_BOTTOM_1), i!(self.update_talk_id),
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

        let _ = config::init(config::Schema::new()
            .field("endless_ng", "exponential", false, None::<String>)
            .field("endless_ng", "exponent_base", 1.2_f64, None::<String>)
            .field("endless_ng", "fix_physical_damage_scaling", true, None::<String>)
            .field("compatibility", "goods_display_id", 67350_i64, None::<String>)
            .field("compatibility", "goods_intensity_id", 67351_i64, None::<String>)
            .field("compatibility", "current_talk_id", 22021102_i64, None::<String>)
            .field("compatibility", "update_talk_id", 22021103_i64, None::<String>)
        );

        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        thread::sleep(time::Duration::from_secs(10));

        let mut goods_data = init_goods();
        let mut message_data = init_message();

        let goods_display_id = config::get_int("compatibility", "goods_display_id").unwrap() as u32;
        let goods_intensity_id = config::get_int("compatibility", "goods_intensity_id").unwrap() as u32;
        let rune_item = 2912;

        message_data.add_message(BND_GOODS_NAME, goods_display_id, "Modify Intensity By:");
        goods_data.add_instance(
            goods_display_id,
            rune_item,
            vec![(EquipParamGoodsField::MaxNum, 9999.0)]
        );

        message_data.add_message(BND_GOODS_NAME, goods_intensity_id, "Grace Ascetic");
        message_data.add_message(BND_GOODS_INFO, goods_intensity_id, "Grace Ascetic Info");
        message_data.add_message(BND_GOODS_CAPTION, goods_intensity_id, "Grace Ascetic Caption");
        goods_data.add_instance(
            goods_intensity_id,
            rune_item,
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

        let current_talk_id = config::get_int("compatibility", "current_talk_id").unwrap() as u32;
        let update_talk_id = config::get_int("compatibility", "update_talk_id").unwrap() as u32;
        
        message_data.add_message(BND_TALK, current_talk_id, "Current Intensity: <?loopCount?>");
        message_data.add_message(BND_TALK, update_talk_id, "Intensity Updated to <?loopCount?>");

        //remove health cap
        let health_cap_aob = "eb 14 81 fa ff ff 07 00 48 8d 44 24 18 4c 8d 44 24 10 49 0f 4e c0 8b 10 89 91 3c 01 00 00";
        let health_cap_offset = 18;
        let health_cap_expected = "49 0f 4e c0";// cmovle rax,r8;
        let health_cap_patch = "49 8b c0 90";   // mov rax,r8; nop;
        perform_patch(health_cap_aob,health_cap_expected,health_cap_patch,health_cap_offset);

        let Ok(solo_param_repository) = (unsafe { SoloParamRepository::instance() }) else { return; };

        if config::get_bool("endless_ng", "fix_physical_damage_scaling").unwrap() {
            fix_physical_damage_scaling(solo_param_repository);
        }
        let (cycle_increase, original_max) = compute_clear_count_cycle_increase(solo_param_repository);

        let mut runner = Box::new(StateRunner::new(
            Intensity { 
                change_sign: 1 ,
                goods_display_id: goods_display_id as i32,
                goods_intensity_id: goods_intensity_id as i32,
                current_talk_id: current_talk_id as i32,
                update_talk_id: update_talk_id as i32,
            },
            TalkScript::new(
                BlockId::none(),
                1000,
                FieldInsHandle { block_id: BlockId::none(), selector: FieldInsSelector(0) },
            ),
        ));

        let exponential = config::get_bool("endless_ng", "exponential").unwrap();
        let exponent_base = config::get_float("endless_ng", "exponent_base").unwrap() as f32;

        // Retrieve games task runner and register a task at frame begin.
        let cs_task = unsafe { CSTaskImp::instance().unwrap() };
        cs_task.run_recurring(
            move |_: &FD4TaskData| { 
                let Ok(game_data_man) = (unsafe { GameDataMan::instance() }) else { return; };
                let Ok(repo) = (unsafe { SoloParamRepository::instance() }) else { return; };

                if game_data_man.ng_lvl > 6 {
                    let over_level = (game_data_man.ng_lvl - 7) as f32;
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
                            if exponential {
                                row1.set_field(field, original_max.get_field(field) + exponent_base.powf(over_level));
                                row2.set_field(field, original_max.get_field(field) + exponent_base.powf(over_level));
                            } else {
                                row1.set_field(field, original_max.get_field(field) + cycle_increase.get_field(field) * over_level);
                                row2.set_field(field, original_max.get_field(field) + cycle_increase.get_field(field) * over_level);
                            }
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