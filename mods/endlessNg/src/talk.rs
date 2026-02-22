use std::time::Duration;

use eldenring::{
    cs::{
        BlockId, CSTaskGroupIndex, CSTaskImp, EzStateInvokeError, FieldInsHandle, FieldInsSelector,
        MenuType, TalkScript, WorldChrMan,
    },
    ez_state::EzStateValue,
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{FromStatic, program::Program, task::*};

use crate::log;

use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY, VK_T};
fn is_key_down(key: VIRTUAL_KEY) -> bool {
    let key_state = unsafe { GetKeyState(key.0 as i32) } as u16;
    key_state & 0x8000 != 0
}

// Command IDs (add these to your constants)
const PLAYER_EQUIPMENT_QUANTITY_CHANGE: i32 = 52; // Command ID for PlayerEquipmentQuantityChange
const CLEAR_QUANTITY_VALUE_OF_CHOOSE_QUANTITY_DIALOG: i32 = 83; // Command ID for ClearQuantityValueOfChooseQuantityDialog
const OPEN_CHOOSE_QUANTITY_DIALOG: i32 = 82; // Command ID for OpenChooseQuantityDialog
const GET_ITEM_HELD_NUM_LIMIT: i32 = 108; // Function ID for GetItemHeldNumLimit
const GET_VALUE_FROM_NUMBER_SELECT_DIALOG: i32 = 62; // Function ID for GetValueFromNumberSelectDialog
const SET_EVENT_FLAG_VALUE: i32 = 147; // Command ID for SetEventFlagValue
const SET_EVENT_FLAG: i32 = 11; // Command ID for SetEventFlag
const OPEN_GENERIC_DIALOG: i32 = 17; // Command ID for OpenGenericDialog

const ITEM_TYPE_GOODS: i32 = 3; // ItemType enum index for Goods
const FLAG_STATE_OFF: i32 = 0;
const FLAG_STATE_ON: i32 = 1; // FlagState enum index for On
const DIALOG_BOX_TYPE_CENTER_BOTTOM_1: i32 = 7; // DialogBoxType enum index for CenterBottom1
const DIALOG_RESULT_LEFT: i32 = 1; // DialogResult enum index for Left
const DIALOG_BOX_STYLE_ORNATE_NO_OPTIONS: i32 = 0; // DialogBoxStyle enum index for OrnateNoOptions

const CHECK_SPECIFIC_PERSON_MENU_IS_OPEN: i32 = 59; // Function ID for CheckSpecificPersonMenuIsOpen
const CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN: i32 = 58; // Function ID for CheckSpecificPersonGenericDialogIsOpen

enum State000001000X84State {
    Idle,
    Enter,
    WaitForDialog,
    WaitForGenericDialog,
    Done,
}

struct State000001000X84 {
    talk_script: TalkScript,
    state: State000001000X84State,
    action1: i32, // FlagState, default ON
}

impl State000001000X84 {
    pub fn step(&mut self) -> Result<bool, EzStateInvokeError> {
        let ts = &mut self.talk_script;

        self.state = match self.state {
            State000001000X84State::Idle => {
                log!("Running state Idle");
                if is_key_down(VK_T) {
                    State000001000X84State::Enter
                } else {
                    State000001000X84State::Idle
                }
            }
            State000001000X84State::Enter => {
                log!("Running state Enter");
                // Get item held num limit
                let limit: i32 = ts.env((
                    GET_ITEM_HELD_NUM_LIMIT,
                    [EzStateValue::Int32(ITEM_TYPE_GOODS), EzStateValue::Int32(67350)],
                ))?.into();

                // PlayerEquipmentQuantityChange(ItemType.Goods, 67350, -GetItemHeldNumLimit(...))
                ts.event((
                    PLAYER_EQUIPMENT_QUANTITY_CHANGE,
                    [
                        EzStateValue::Int32(ITEM_TYPE_GOODS),
                        EzStateValue::Int32(67350),
                        EzStateValue::Int32(-limit),
                    ],
                ))?;

                // PlayerEquipmentQuantityChange(ItemType.Goods, 67350, GetItemHeldNumLimit(...))
                ts.event((
                    PLAYER_EQUIPMENT_QUANTITY_CHANGE,
                    [
                        EzStateValue::Int32(ITEM_TYPE_GOODS),
                        EzStateValue::Int32(67350),
                        EzStateValue::Int32(limit),
                    ],
                ))?;

                // ClearQuantityValueOfChooseQuantityDialog()
                ts.event(CLEAR_QUANTITY_VALUE_OF_CHOOSE_QUANTITY_DIALOG)?;

                // OpenChooseQuantityDialog(67350, 22021102)
                ts.event((
                    OPEN_CHOOSE_QUANTITY_DIALOG,
                    [EzStateValue::Int32(67350), EzStateValue::Int32(22021102)],
                ))?;

                State000001000X84State::WaitForDialog
            }

            State000001000X84State::WaitForDialog => {
                log!("Running state WaitForDialog");
                // assert not (CheckSpecificPersonMenuIsOpen(13, 0) == true
                //             and not CheckSpecificPersonGenericDialogIsOpen(0))
                let menu_open: i32 = ts.env((
                    CHECK_SPECIFIC_PERSON_MENU_IS_OPEN,
                    [EzStateValue::Int32(13), EzStateValue::Int32(0)],
                ))?.into();

                let dialog_open: i32 = ts.env((
                    CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN,
                    [EzStateValue::Int32(0)],
                ))?.into();

                // Keep waiting while the dialog is still open
                if menu_open == 1 && dialog_open == 0 {
                    return Ok(false); // still waiting
                }

                // GetValueFromNumberSelectDialog()
                let value: i32 = ts.env(GET_VALUE_FROM_NUMBER_SELECT_DIALOG)?.into();

                if value >= 0 {
                    // SetEventFlagValue(1051439332, 32, value)
                    ts.event((
                        SET_EVENT_FLAG_VALUE,
                        [
                            EzStateValue::Int32(1051439332),
                            EzStateValue::Int32(32),
                            EzStateValue::Int32(value),
                        ],
                    ))?;

                    // SetEventFlag(1051439331, action1)
                    ts.event((
                        SET_EVENT_FLAG,
                        [
                            EzStateValue::Int32(1051439331),
                            EzStateValue::Int32(self.action1),
                        ],
                    ))?;

                    // SetEventFlag(1051439330, FlagState.On)
                    ts.event((
                        SET_EVENT_FLAG,
                        [
                            EzStateValue::Int32(1051439330),
                            EzStateValue::Int32(FLAG_STATE_ON),
                        ],
                    ))?;
                }
                // else: pass — nothing to do

                // PlayerEquipmentQuantityChange(ItemType.Goods, 67350, -GetItemHeldNumLimit(...))
                let limit: i32 = ts.env((
                    GET_ITEM_HELD_NUM_LIMIT,
                    [EzStateValue::Int32(ITEM_TYPE_GOODS), EzStateValue::Int32(67350)],
                ))?.into();

                ts.event((
                    PLAYER_EQUIPMENT_QUANTITY_CHANGE,
                    [
                        EzStateValue::Int32(ITEM_TYPE_GOODS),
                        EzStateValue::Int32(67350),
                        EzStateValue::Int32(-limit),
                    ],
                ))?;

                // OpenGenericDialog(DialogBoxType.CenterBottom1, 22021103,
                //                   DialogResult.Left, DialogBoxStyle.OrnateNoOptions, 1)
                ts.event((
                    OPEN_GENERIC_DIALOG,
                    [
                        EzStateValue::Int32(DIALOG_BOX_TYPE_CENTER_BOTTOM_1),
                        EzStateValue::Int32(22021103),
                        EzStateValue::Int32(DIALOG_RESULT_LEFT),
                        EzStateValue::Int32(DIALOG_BOX_STYLE_ORNATE_NO_OPTIONS),
                        EzStateValue::Int32(1),
                    ],
                ))?;

                State000001000X84State::WaitForGenericDialog
            }

            State000001000X84State::WaitForGenericDialog => {
                log!("Running state WaitForGenericDialog");
                // assert not CheckSpecificPersonGenericDialogIsOpen(0)
                let dialog_open: i32 = ts.env((
                    CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN,
                    [EzStateValue::Int32(0)],
                ))?.into();

                if dialog_open == 1 {
                    return Ok(false); // still waiting
                }

                State000001000X84State::Done
            }

            State000001000X84State::Done => {
                log!("Running state Done");
                return Ok(true); // return 1
            }
        };

        Ok(false)
    }
}

unsafe impl Send for State000001000X84 {}

pub fn test() {
    let mut demo = Box::new(State000001000X84 {
        talk_script: TalkScript::new(
            BlockId::none(),
            1000,
            FieldInsHandle {
                block_id: BlockId::none(),
                selector: FieldInsSelector(0),
            },
        ),
        state: State000001000X84State::Idle,
        action1: 0
    });

    let cs_task = unsafe { CSTaskImp::instance().unwrap() };
    cs_task.run_recurring(
        move |_: &FD4TaskData| {
            if let Ok(world_chr_man) = unsafe { WorldChrMan::instance() }
                && let Some(ref mut main_player) = world_chr_man.main_player
            {
                demo.talk_script.npc_talk.base.field_ins_handle =
                    main_player.chr_ins.field_ins_handle;

                if let Err(e) = demo.step() {
                    log!("{:?}", e);
                    demo.state = State000001000X84State::Idle;
                }
            }
        },
        CSTaskGroupIndex::FrameBegin,
    );
}