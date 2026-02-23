#![allow(dead_code)]

use eldenring::{
    cs::{
        EzStateInvokeError, TalkScript
    },
};

// ========================
// Commands
// ========================
pub const DEBUG_EVENT: i32 = 0;
pub const TALK_TO_PLAYER: i32 = 1;
pub const INVOKE_EVENT: i32 = 2;
pub const STOP_ATTACKING: i32 = 3;
pub const ATTACK: i32 = 4;
pub const REMOVE_MY_AGGRO: i32 = 5;
pub const DISPLAY_ONE_LINE_HELP: i32 = 6;
pub const TURN_TO_FACE_PLAYER: i32 = 7;
pub const FORCE_END_TALK: i32 = 8;
pub const CLEAR_TALK_PROGRESS_DATA: i32 = 9;
pub const SHOW_SHOP_MESSAGE: i32 = 10;
pub const SET_EVENT_FLAG: i32 = 11;
pub const CLOSE_SHOP_MESSAGE: i32 = 12;
pub const OPEN_CAMP_MENU: i32 = 13;
pub const CLOSE_CAMP_MENU: i32 = 14;
pub const CHANGE_TEAM_TYPE: i32 = 15;
pub const SET_DEFAULT_TEAM_TYPE: i32 = 16;
pub const OPEN_GENERIC_DIALOG: i32 = 17;
pub const FORCE_CLOSE_GENERIC_DIALOG: i32 = 18;
pub const ADD_TALK_LIST_DATA: i32 = 19;
pub const CLEAR_TALK_LIST_DATA: i32 = 20;
pub const REQUEST_MOVIE_PLAYBACK: i32 = 21;
pub const OPEN_REGULAR_SHOP: i32 = 22;
pub const OPEN_REPAIR_SHOP: i32 = 23;
pub const OPEN_ENHANCE_SHOP: i32 = 24;
pub const OPEN_HUMANITY_MENU: i32 = 25;
pub const OPEN_MAGIC_SHOP: i32 = 26;
pub const OPEN_MIRACLE_SHOP: i32 = 27;
pub const OPEN_MAGIC_EQUIP: i32 = 28;
pub const OPEN_MIRACLE_EQUIP: i32 = 29;
pub const OPEN_REPOSITORY: i32 = 30;
pub const OPEN_SOUL: i32 = 31;
pub const CLOSE_MENU: i32 = 32;
pub const SET_EVENT_FLAG_RANGE: i32 = 33;
pub const OPEN_DEPOSITORY: i32 = 34;
pub const CLEAR_TALK_ACTION_STATE: i32 = 35;
pub const CLEAR_TALK_DISABLED_STATE: i32 = 36;
pub const SET_TALK_DISABLE_STATE_MAX_DURATION: i32 = 37;
pub const SET_UPDATE_DISTANCE: i32 = 38;
pub const CLEAR_PLAYER_DAMAGE_INFO: i32 = 39;
pub const START_WARP_MENU_INIT: i32 = 41;
pub const START_BONFIRE_ANIM_LOOP: i32 = 42;
pub const END_BONFIRE_KINDLE_ANIM_LOOP: i32 = 43;
pub const OPEN_SELL_SHOP: i32 = 46;
pub const CHANGE_PLAYER_STAT: i32 = 47;
pub const OPEN_EQUIPMENT_CHANGE_OF_PURPOSE_SHOP: i32 = 48;
pub const COMBINE_MENU_FLAG_AND_EVENT_FLAG: i32 = 49;
pub const REQUEST_SAVE: i32 = 50;
pub const CHANGE_MOTION_OFFSET_ID: i32 = 51;
pub const PLAYER_EQUIPMENT_QUANTITY_CHANGE: i32 = 52;
pub const REQUEST_UNLOCK_TROPHY: i32 = 53;
pub const ENTER_BONFIRE_EVENT_RANGE: i32 = 54;
pub const SET_AQUITTAL_COST_MESSAGE_TAG: i32 = 55;
pub const SUBTRACT_ACQUITTAL_COST_FROM_PLAYER_SOULS: i32 = 56;
pub const SHUFFLE_RNG_SEED: i32 = 57;
pub const SET_RNG_SEED: i32 = 58;
pub const REPLACE_TOOL: i32 = 59;
pub const BREAK_COVENANT_PLEDGE: i32 = 60;
pub const PLAYER_RESPAWN: i32 = 61;
pub const GIVE_SP_EFFECT_TO_PLAYER: i32 = 62;
pub const SHOW_GRANDIOSE_TEXT_PRESENTATION: i32 = 63;
pub const ADD_IZALITH_RANKING_POINTS: i32 = 64;
pub const OPEN_ITEM_ACQUISITION_MENU: i32 = 65;
pub const ACQUIRE_GESTURE_OLD: i32 = 66;
pub const FORCE_CLOSE_MENU: i32 = 67;
pub const SET_TALK_TIME: i32 = 68;
pub const COLLECT_JUST_PYROMANCY_FLAME: i32 = 69;
pub const OPEN_ARENA_RANKING: i32 = 70;
pub const REPORT_CONVERSATION_END_TO_HAVOK_BEHAVIOR: i32 = 71;
pub const OPEN_CONVERSATION_CHOICES_MENU: i32 = 76;
pub const STOP_EVENT_ANIM_WITHOUT_FORCING_CONVERSATION_END: i32 = 80;
pub const OPEN_CHARA_MAKE_MENU: i32 = 81;
pub const OPEN_CHOOSE_QUANTITY_DIALOG: i32 = 82;
pub const CLEAR_QUANTITY_VALUE_OF_CHOOSE_QUANTITY_DIALOG: i32 = 83;
pub const SET_WORK_VALUE: i32 = 100;
pub const UPDATE_PLAYER_RESPAWN_POINT: i32 = 101;
pub const SET_MESSAGE_TAG_VALUE: i32 = 102;
pub const TURN_CHARACTER_TO_FACE_ENTITY: i32 = 103;
pub const AWARD_ITEM_LOT: i32 = 104;
pub const OPEN_ESTUS_ALLOT_MENU: i32 = 105;
pub const ADD_RANKING_POINTS: i32 = 106;
pub const OPEN_HOLLOW_LEVEL_UP_MENU: i32 = 107;
pub const ESTUS_ALLOCATION_UPDATE: i32 = 108;
pub const BONFIRE_ACTIVATION: i32 = 109;
pub const OPEN_TRANSPOSITION_SHOP: i32 = 111;
pub const SET_BONFIRE_MENU_DATA: i32 = 112;
pub const REALLOCATE_ATTRIBUTES: i32 = 113;
pub const UNDEAD_MATCH: i32 = 114;
pub const OPEN_BONFIRE_MENU: i32 = 115;
pub const REQUEST_ANIMATION: i32 = 123;
pub const OPEN_PHYSICK_MENU: i32 = 130;
pub const ACQUIRE_GESTURE: i32 = 131;
pub const OPEN_DRAGON_COMMUNION_SHOP: i32 = 135;
pub const OPEN_TAILORING_SHOP: i32 = 142;
pub const OPEN_ASH_OF_WAR_SHOP: i32 = 143;
pub const OPEN_PUPPET_SHOP: i32 = 144;
pub const OPEN_DUPE_SHOP: i32 = 146;
pub const SET_EVENT_FLAG_VALUE: i32 = 147;
pub const OPEN_CHAMPIONS_EQUIPMENT_SHOP: i32 = 148;

// ========================
// Functions
// ========================
pub const GET_WHETHER_ENEMIES_ARE_NEARBY: i32 = 0;
pub const GET_DISTANCE_TO_PLAYER: i32 = 1;
pub const HAS_TALK_ENDED: i32 = 2;
pub const CHECK_SELF_DEATH: i32 = 3;
pub const IS_PLAYER_TALKING_TO_ME: i32 = 4;
pub const IS_ATTACKED_BY_SOMEONE: i32 = 5;
pub const GET_SELF_HP: i32 = 6;
pub const GET_DISTANCE_FROM_ENEMY: i32 = 7;
pub const GET_RELATIVE_ANGLE_BETWEEN_PLAYER_AND_SELF: i32 = 8;
pub const IS_PLAYER_ATTACKING: i32 = 9;
pub const GET_RELATIVE_ANGLE_BETWEEN_SELF_AND_PLAYER: i32 = 10;
pub const IS_TALK_IN_PROGRESS: i32 = 11;
pub const GET_TALK_INTERRUPT_REASON: i32 = 12;
pub const GET_SHOP_CONDITION: i32 = 13;
pub const GET_ONE_LINE_HELP_STATUS: i32 = 14;
pub const GET_EVENT_FLAG: i32 = 15;
pub const DOES_PLAYER_HAVE_ITEM: i32 = 16;
pub const DOES_PLAYER_HAVE_ITEM_EQUIPPED: i32 = 17;
pub const IS_FIGHTING_ALONE: i32 = 18;
pub const IS_CLIENT_PLAYER: i32 = 19;
pub const IS_CAMP_MENU_OPEN: i32 = 20;
pub const IS_GENERIC_DIALOG_OPEN: i32 = 21;
pub const GET_GENERIC_DIALOG_BUTTON_RESULT: i32 = 22;
pub const GET_TALK_LIST_ENTRY_RESULT: i32 = 23;
pub const IS_MOVIE_PLAYING: i32 = 24;
pub const IS_MENU_OPEN: i32 = 25;
pub const IS_CHARACTER_DISABLED: i32 = 26;
pub const IS_PLAYER_DEAD: i32 = 27;
pub const DID_YOU_DO_SOMETHING_IN_THE_MENU: i32 = 28;
pub const GET_STATUS: i32 = 29;
pub const IS_PLAYER_MOVING_A_CERTAIN_DISTANCE: i32 = 30;
pub const IS_TALKING_TO_SOMEONE_ELSE: i32 = 31;
pub const HAS_DISABLE_TALK_PERIOD_ELAPSED: i32 = 32;
pub const HAS_PLAYER_BEEN_ATTACKED: i32 = 33;
pub const GET_PLAYER_Y_DISTANCE: i32 = 34;
pub const GET_PLAYER_CHR_TYPE: i32 = 35;
pub const CAN_I_GO_TO_NEXT_TALK_BLOCK: i32 = 36;
pub const COMPARE_BONFIRE_STATE: i32 = 37;
pub const COMPARE_BONFIRE_LEVEL: i32 = 38;
pub const COMPARE_PARENT_BONFIRE: i32 = 39;
pub const BONFIRE_REGISTRATION0: i32 = 40;
pub const BONFIRE_REGISTRATION1: i32 = 41;
pub const BONFIRE_REGISTRATION2: i32 = 42;
pub const BONFIRE_REGISTRATION3: i32 = 43;
pub const BONFIRE_REGISTRATION4: i32 = 44;
pub const COMPARE_PLAYER_STAT: i32 = 45;
pub const RELATIVE_ANGLE_BETWEEN_TWO_PLAYERS_SPECIFY_AXIS: i32 = 46;
pub const COMPARE_PLAYER_INVENTORY_NUMBER: i32 = 47;
pub const IS_PLAYER_CURRENT_WEAPON_DAMAGED: i32 = 48;
pub const COMPARE_PLAYER_ACQUITTAL_PRICE: i32 = 49;
pub const COMPARE_RNG_VALUE: i32 = 50;
pub const WAS_WARP_MENU_DESTINATION_SELECTED: i32 = 51;
pub const IS_MULTIPLAYER_IN_PROGRESS: i32 = 52;
pub const IS_TALK_EXCLUSIVE_MENU_OPEN: i32 = 53;
pub const IS_RANKING_MENU_OPEN: i32 = 54;
pub const GET_PLAYER_REMAINING_HP: i32 = 55;
pub const CHECK_ACTION_BUTTON_AREA: i32 = 56;
pub const CHECK_SPECIFIC_PERSON_TALK_HAS_ENDED: i32 = 57;
pub const CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN: i32 = 58;
pub const CHECK_SPECIFIC_PERSON_MENU_IS_OPEN: i32 = 59;
pub const DOES_SELF_HAVE_SP_EFFECT: i32 = 60;
pub const DOES_PLAYER_HAVE_SP_EFFECT: i32 = 61;
pub const GET_VALUE_FROM_NUMBER_SELECT_DIALOG: i32 = 62;
pub const GET_WORK_VALUE: i32 = 100;
pub const GET_EVENT_FLAG_VALUE: i32 = 101;
pub const GET_CURRENT_STATE_ELAPSED_FRAMES: i32 = 102;
pub const GET_CURRENT_STATE_ELAPSED_TIME: i32 = 103;
pub const GET_PLAYER_STAT: i32 = 104;
pub const GET_LEVEL_UP_SOUL_COST: i32 = 105;
pub const GET_WHETHER_CHR_TURN_ANIM_HAS_ENDED: i32 = 106;
pub const GET_WHETHER_CHR_EVENT_ANIM_HAS_ENDED: i32 = 107;
pub const GET_ITEM_HELD_NUM_LIMIT: i32 = 108;
pub const GET_ESTUS_ALLOCATION: i32 = 109;
pub const GET_TOTAL_BONFIRE_LEVEL: i32 = 110;
pub const GET_IS_ONLINE: i32 = 111;
pub const GET_IS_NET_PENALIZED_FOR_ARENA: i32 = 112;

// ========================
// Enums
// ========================

// CompareType
pub const COMPARE_TYPE_EQUAL: i32 = 0;
pub const COMPARE_TYPE_NOT_EQUAL: i32 = 1;
pub const COMPARE_TYPE_GREATER: i32 = 2;
pub const COMPARE_TYPE_LESS: i32 = 3;
pub const COMPARE_TYPE_GREATER_OR_EQUAL: i32 = 4;
pub const COMPARE_TYPE_LESS_OR_EQUAL: i32 = 5;

// ChangeType
pub const CHANGE_TYPE_ADD: i32 = 0;
pub const CHANGE_TYPE_SUBTRACT: i32 = 1;
pub const CHANGE_TYPE_SET: i32 = 5;

// ChrType
pub const CHR_TYPE_HOLLOW: i32 = 8;

// DialogBoxType
pub const DIALOG_BOX_TYPE_CENTER_MIDDLE_DIM_SCREEN1: i32 = 1;
pub const DIALOG_BOX_TYPE_CENTER_MIDDLE_DIM_SCREEN2: i32 = 2;
pub const DIALOG_BOX_TYPE_CENTER_BOTTOM_1: i32 = 7;
pub const DIALOG_BOX_TYPE_CENTER_BOTTOM_2: i32 = 8;

// DialogResult
pub const DIALOG_RESULT_CANCEL: i32 = 0;
pub const DIALOG_RESULT_LEFT: i32 = 1;
pub const DIALOG_RESULT_RIGHT: i32 = 2;
pub const DIALOG_RESULT_LEAVE: i32 = 3;

// DialogBoxStyle
pub const DIALOG_BOX_STYLE_ORNATE_NO_OPTIONS: i32 = 0;
pub const DIALOG_BOX_STYLE_DIALOG_OPTIONS: i32 = 1;
pub const DIALOG_BOX_STYLE_ORNATE_OPTIONS: i32 = 2;
pub const DIALOG_BOX_STYLE_ORNATE_YES_OPTION: i32 = 3;
pub const DIALOG_BOX_STYLE_UNK: i32 = 4;

// EnhanceType
pub const ENHANCE_TYPE_UNLIMITED_RANGE: i32 = 0;
pub const ENHANCE_TYPE_LIMITED_RANGE: i32 = 1;

// EstusType
pub const ESTUS_TYPE_HP: i32 = 0;
pub const ESTUS_TYPE_FP: i32 = 1;

// FlagState
pub const FLAG_STATE_OFF: i32 = 0;
pub const FLAG_STATE_ON: i32 = 1;

// ItemType
pub const ITEM_TYPE_WEAPON: i32 = 0;
pub const ITEM_TYPE_PROTECTOR: i32 = 1;
pub const ITEM_TYPE_ACCESSORY: i32 = 2;
pub const ITEM_TYPE_GOODS: i32 = 3;

// MenuType
pub const MENU_TYPE_TALK: i32 = 11;
pub const MENU_TYPE_REPAIR: i32 = 12;
pub const MENU_TYPE_REINFORCEMENT: i32 = 13;
pub const MENU_TYPE_LEVEL_UP: i32 = 23;
pub const MENU_TYPE_ATTUNEMENT: i32 = 25;
pub const MENU_TYPE_STORAGE: i32 = 26;
pub const MENU_TYPE_CHARACTER_EDIT: i32 = 30;
pub const MENU_TYPE_COVENANT_OFFERING: i32 = 31;
pub const MENU_TYPE_WARP: i32 = 36;
pub const MENU_TYPE_UNK: i32 = 53;
pub const MENU_TYPE_BONFIRE: i32 = 63;

// PlayerStat
pub const PLAYER_STAT_VIGOR: i32 = 0;
pub const PLAYER_STAT_MIND: i32 = 1;
pub const PLAYER_STAT_ENDURANCE: i32 = 2;
pub const PLAYER_STAT_STRENGTH: i32 = 3;
pub const PLAYER_STAT_DEXTERITY: i32 = 4;
pub const PLAYER_STAT_INTELLIGENCE: i32 = 5;
pub const PLAYER_STAT_FAITH: i32 = 6;
pub const PLAYER_STAT_ARCANE: i32 = 7;
pub const PLAYER_STAT_RUNES_COLLECTED: i32 = 8;
pub const PLAYER_STAT_TOTAL_GET_RUNES: i32 = 9;
pub const PLAYER_STAT_HUMANITY: i32 = 10;
pub const PLAYER_STAT_COVENANT_TYPE: i32 = 11;
pub const PLAYER_STAT_GENDER: i32 = 12;
pub const PLAYER_STAT_EQUIPPED_COVENANT_POINTS: i32 = 15;
pub const PLAYER_STAT_EQUIPPED_COVENANT_LEVEL: i32 = 16;
pub const PLAYER_STAT_BLADE_OF_DARKMOON_POINTS: i32 = 17;
pub const PLAYER_STAT_WARRIOR_OF_SUNLIGHT_POINTS: i32 = 18;
pub const PLAYER_STAT_MOUND_MAKERS_POINTS: i32 = 19;
pub const PLAYER_STAT_SPEARS_OF_THE_CHURCH_POINTS: i32 = 20;
pub const PLAYER_STAT_ROSARIAS_FINGERS_POINTS: i32 = 21;
pub const PLAYER_STAT_WATCHDOGS_OF_FARRON_POINTS: i32 = 22;
pub const PLAYER_STAT_ALDRICH_FAITHFUL_POINTS: i32 = 23;
pub const PLAYER_STAT_BLADE_OF_DARKMOON_LEVEL: i32 = 24;
pub const PLAYER_STAT_WARRIOR_OF_SUNLIGHT_LEVEL: i32 = 25;
pub const PLAYER_STAT_MOUND_MAKERS_LEVEL: i32 = 26;
pub const PLAYER_STAT_SPEARS_OF_THE_CHURCH_LEVEL: i32 = 27;
pub const PLAYER_STAT_ROSARIAS_FINGERS_LEVEL: i32 = 28;
pub const PLAYER_STAT_WATCHDOGS_OF_FARRON_LEVEL: i32 = 29;
pub const PLAYER_STAT_ALDRICH_FAITHFUL_LEVEL: i32 = 30;
pub const PLAYER_STAT_HOLLOWING: i32 = 31;
pub const PLAYER_STAT_HOLLOWING_COVENANT_LEVEL: i32 = 32;
pub const PLAYER_STAT_RUNE_LEVEL: i32 = 33;
pub const PLAYER_STAT_REMAINING_YOEL_LEVELS: i32 = 34;
pub const PLAYER_STAT_WAY_OF_BLUE_POINTS: i32 = 35;
pub const PLAYER_STAT_WAY_OF_BLUE_LEVEL: i32 = 36;
pub const PLAYER_STAT_BLUE_SENTINELS_POINTS: i32 = 37;
pub const PLAYER_STAT_BLUE_SENTINELS_LEVEL: i32 = 38;

// TalkOptionsType
pub const TALK_OPTIONS_TYPE_OLD: i32 = 0;
pub const TALK_OPTIONS_TYPE_REGULAR: i32 = 1;

// Trophy
pub const TROPHY_ALL_ACHIEVEMENTS: i32 = 0;
pub const TROPHY_LINK_THE_FIRST_FLAME: i32 = 1;
pub const TROPHY_END_OF_FIRE: i32 = 2;
pub const TROPHY_USURPATION_OF_FIRE: i32 = 3;
pub const TROPHY_ABYSS_WATCHERS: i32 = 4;
pub const TROPHY_YHORM_THE_GIANT: i32 = 5;
pub const TROPHY_ALDRICH: i32 = 6;
pub const TROPHY_LORIAN_AND_LORDELLE: i32 = 7;
pub const TROPHY_SISTER_FRAMPT: i32 = 8;
pub const TROPHY_OCELOT: i32 = 9;

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

unsafe impl<T: StateMachine> Send for StateRunner<T> {}

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