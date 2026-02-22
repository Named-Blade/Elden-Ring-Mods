use eldenring::{
    cs::{
        BlockId, CSTaskGroupIndex, CSTaskImp, EzStateInvokeError, FieldInsHandle, FieldInsSelector,
        TalkScript, WorldChrMan, GameDataMan
    },
    ez_state::EzStateValue,
    fd4::FD4TaskData
};
use fromsoftware_shared::{FromStatic, task::*};

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
const OPEN_GENERIC_DIALOG: i32 = 17; // Command ID for OpenGenericDialog

const ITEM_TYPE_GOODS: i32 = 3; // ItemType enum index for Goods
const DIALOG_BOX_TYPE_CENTER_BOTTOM_1: i32 = 7; // DialogBoxType enum index for CenterBottom1
const DIALOG_RESULT_LEFT: i32 = 1; // DialogResult enum index for Left
const DIALOG_BOX_STYLE_ORNATE_NO_OPTIONS: i32 = 0; // DialogBoxStyle enum index for OrnateNoOptions

const CHECK_SPECIFIC_PERSON_MENU_IS_OPEN: i32 = 59; // Function ID for CheckSpecificPersonMenuIsOpen
const CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN: i32 = 58; // Function ID for CheckSpecificPersonGenericDialogIsOpen

/// A trait for defining state machine states with associated data.
pub trait StateMachine: Sized {
    type State: Default;
    
    fn step_state(
        &mut self,
        state: Self::State,
        ts: &mut TalkScript,
    ) -> Result<Transition<Self::State>, EzStateInvokeError>;
}

/// What the state machine should do after a state runs.
pub enum Transition<S> {
    /// Move to a new state (or stay on the same one)
    Next(S),
    /// Finished — return true
    Done,
    /// Not finished yet — return false without changing state
    Wait(S)
}

/// Generic runner wrapping any StateMachine implementation.
pub struct StateRunner<M: StateMachine> {
    pub data: M,
    pub talk_script: TalkScript,
    pub state: M::State,
}

impl<M: StateMachine> StateRunner<M> {
    pub fn new(data: M, talk_script: TalkScript) -> Self {
        Self {
            data,
            talk_script,
            state: M::State::default(),
        }
    }

    pub fn step(&mut self) -> Result<bool, EzStateInvokeError> {
        let state = std::mem::take(&mut self.state);

        match self.data.step_state(state, &mut self.talk_script)? {
            Transition::Next(next) => {
                self.state = next;
                Ok(false)
            }
            Transition::Done => Ok(true),
            Transition::Wait(current) => {  // <-- return the state back
                self.state = current;
                Ok(false)
            }
        }
    }
}

#[derive(Default, Debug)]
enum IntensityState {
    #[default]
    Idle,
    Enter,
    WaitForDialog,
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
                if is_key_down(VK_T) { Next(IntensityState::Enter) } else { Transition::<IntensityState>::Wait(IntensityState::Idle) }
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
                        game_data_man.ng_lvl -= value as u32;
                    }  else {
                        game_data_man.ng_lvl += value as u32;
                    }
                    log!("set ng level to {}", game_data_man.ng_lvl);
                }

                let limit: i32 = env!((GET_ITEM_HELD_NUM_LIMIT, [i!(ITEM_TYPE_GOODS), i!(67350)])).into();
                event!((PLAYER_EQUIPMENT_QUANTITY_CHANGE, [i!(ITEM_TYPE_GOODS), i!(67350), i!(-limit)]));
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

pub fn test() {
    let mut runner = Box::new(StateRunner::new(
        Intensity { change_sign: 1 },
        TalkScript::new(
            BlockId::none(),
            1000,
            FieldInsHandle { block_id: BlockId::none(), selector: FieldInsSelector(0) },
        ),
    ));

    let cs_task = unsafe { CSTaskImp::instance().unwrap() };
    cs_task.run_recurring(
        move |_: &FD4TaskData| {
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
}