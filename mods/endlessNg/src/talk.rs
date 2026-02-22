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

// Command IDs (add these to your constants)
const PLAYER_EQUIPMENT_QUANTITY_CHANGE: i32 = 0;
const CLEAR_QUANTITY_VALUE_OF_CHOOSE_QUANTITY_DIALOG: i32 = 0;
const OPEN_CHOOSE_QUANTITY_DIALOG: i32 = 0;
const GET_ITEM_HELD_NUM_LIMIT: i32 = 0;
const GET_VALUE_FROM_NUMBER_SELECT_DIALOG: i32 = 0;
const SET_EVENT_FLAG_VALUE: i32 = 0;
const SET_EVENT_FLAG: i32 = 0;
const OPEN_GENERIC_DIALOG: i32 = 0;

const ITEM_TYPE_GOODS: i32 = 3; // adjust to match ItemType.Goods
const FLAG_STATE_ON: i32 = 1;
const DIALOG_BOX_TYPE_CENTER_BOTTOM_1: i32 = 0;
const DIALOG_RESULT_LEFT: i32 = 0;
const DIALOG_BOX_STYLE_ORNATE_NO_OPTIONS: i32 = 0;

const CHECK_SPECIFIC_PERSON_MENU_IS_OPEN: i32 = 59;
const CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN: i32 = 58;

enum State000001000X84State {
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
            State000001000X84State::Enter => {
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
                return Ok(true); // return 1
            }
        };

        Ok(false)
    }
}