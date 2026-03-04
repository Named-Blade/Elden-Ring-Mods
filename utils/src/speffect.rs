use eldenring::param::SP_EFFECT_PARAM_ST;
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;
use winhook::HookHandle;

use crate::hook::*;
use crate::log;
use crate::into_value::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SpEffectParamField {
    Soul,
    EffectEndurance
}

#[allow(dead_code)]
impl SpEffectParamField {
    pub const ALL: [SpEffectParamField; 2] = [
        Self::Soul,
        Self::EffectEndurance,
    ];

    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            SpEffectParamField::Soul => "soul",
            SpEffectParamField::EffectEndurance => "effect_endurance",
        }
    }
}

pub trait SpEffectParamFieldAccess {
    #[allow(dead_code)]
    fn get_field(&self, field: SpEffectParamField) -> f32;
    fn set_field(&mut self, field: SpEffectParamField, value: impl IntoValue);
}

impl SpEffectParamFieldAccess for SP_EFFECT_PARAM_ST {
    fn get_field(&self, field: SpEffectParamField) -> f32 {
        match field {
            SpEffectParamField::Soul => self.soul() as f32,
            SpEffectParamField::EffectEndurance => self.effect_endurance(),
        }
    }

    fn set_field(&mut self, field: SpEffectParamField, value: impl IntoValue) {
        match field {
            SpEffectParamField::Soul => self.set_soul(value.to_i32()),
            SpEffectParamField::EffectEndurance => self.set_effect_endurance(value.to_f32()),
        }
    }
}

pub const GET_SP_EFFECT_AOB: &str = "48 8d 48 e8 48 89 78 e8 8b ea 40 88 78 f4 e8 ? ? ? ? 48 8b 44 24 20 48 85 c0";
pub const GET_SP_EFFECT_OFFSET: usize = 15;

#[repr(C, packed)]
pub struct GetSpEffectResult {
    pub row: *mut SP_EFFECT_PARAM_ST,
    pub id: u32,
    pub _1: [u8; 0x4],
}

pub type GetSpEffectType = unsafe extern "C" fn(
    result: *mut GetSpEffectResult,
    id: u32,
);

pub static GET_SP_EFFECT_ORIGINAL_HOLDER: OnceLock<GetSpEffectType> = OnceLock::new();

type SpEffectFieldsPair = (SpEffectParamField, f32);
pub struct SpEffectInstance {
    sp_effect: Option<Box<SP_EFFECT_PARAM_ST>>,
    source: u32,
    fields: Vec<SpEffectFieldsPair>
}

impl SpEffectInstance {
    pub fn set_sp_effect(&mut self, sp_effect: &SP_EFFECT_PARAM_ST) {
        let mut my_sp_effect: SP_EFFECT_PARAM_ST = sp_effect.clone();
        for (field,value) in self.fields.iter() {
            my_sp_effect.set_field(*field, *value);
        }
        self.sp_effect = Some(Box::new(my_sp_effect));
    }

    pub fn get_sp_effect(&self) -> Option<*const SP_EFFECT_PARAM_ST> {
        let Some(ref sp_effect) = self.sp_effect else { return None };
        return Some(&*(*sp_effect) as *const SP_EFFECT_PARAM_ST);
    }

    pub fn get_source(&self) -> u32 {
        self.source
    }
}

pub struct SpEffectData {
    map: HashMap<u32, SpEffectInstance>,
    hook: Option<HookHandle>
}

impl SpEffectData {
    pub fn add_instance(&mut self, inst: u32, source: u32, fields: Vec<SpEffectFieldsPair>) {
        if !self.map.contains_key(&inst) {
            log!("added sp effect of id {}", inst);
            self.map.insert(inst, SpEffectInstance{
                sp_effect: None,
                source: source,
                fields: fields.clone()
            });
        }
    }

    pub fn get_instance(&self, inst: u32) -> Option<&SpEffectInstance> {
        let Some(sp_effect) = self.map.get(&inst) else {return None};
        return Some(sp_effect);
    }

    pub fn set_hook(&mut self, hook: HookHandle) {
        self.hook = Some(hook);
    }
}

pub struct SpEffectContainer {
    arc: Arc<Mutex<SpEffectData>>
}

impl SpEffectContainer {
    pub fn add_instance(&mut self, inst: u32, source: u32, fields: Vec<SpEffectFieldsPair>) {
        let mut data = self.arc.lock().unwrap();
        data.add_instance(inst, source, fields);
    }
}

pub fn init_sp_effect() -> SpEffectContainer {
    let sp_effect_data = Arc::new(Mutex::new(SpEffectData {
        map: HashMap::new(),
        hook: None,
    }));

    let closure_data = Arc::clone(&sp_effect_data);

    let hook_handle_sp_effect = make_installer_from_call_aob::<GetSpEffectType>(
        GET_SP_EFFECT_AOB,
        GET_SP_EFFECT_OFFSET,
        &GET_SP_EFFECT_ORIGINAL_HOLDER,
    )
    .unwrap()
    .install_mut({
        move |original| {
            let closure_data = Arc::clone(&closure_data);
            move |result, id| {
                let data = closure_data.lock().unwrap();
                if let Some(sp_effect_instance) = data.get_instance(id) {
                    unsafe {
                        if let Some(sp_effect) = sp_effect_instance.get_sp_effect() {
                            (*result).row = sp_effect as *mut SP_EFFECT_PARAM_ST;
                        } else {
                            original(result, sp_effect_instance.get_source());
                            let ptr = sp_effect_instance as *const SpEffectInstance;
                            (*(ptr as usize as *mut SpEffectInstance)).set_sp_effect(&*(*result).row);
                            log!("instantiated sp effect of id {}", id);
                            (*result).row = sp_effect_instance.get_sp_effect().unwrap() as *mut SP_EFFECT_PARAM_ST;
                        }
                        (*result).id = id;
                    }
                } else {
                    unsafe { original(result,id) };
                }
            }
        }
    })
    .unwrap();

    unsafe { hook_handle_sp_effect.enable(true) };
    {   
        let mut data = sp_effect_data.lock().unwrap();
        data.hook = Some(hook_handle_sp_effect);
    }

    return SpEffectContainer{arc: sp_effect_data};
}