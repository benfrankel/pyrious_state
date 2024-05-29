use bevy_ecs::system::{
    lifetimeless::{SRes, SResMut},
    Resource, SystemParamItem,
};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectResource;

use crate::{
    pattern::StatePattern,
    state::RawState,
    storage::{GetStateStorage, SetStateStorage, StateStorage},
};

// TODO: Update this comment and the other one.
// The mutable half of the double-buffered state.
#[derive(Resource, Debug)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Resource)
)]
pub struct StateSlot<S: RawState>(pub Option<S>);

impl<S: RawState> StateStorage<S> for StateSlot<S> {}

impl<S: RawState> GetStateStorage<S> for StateSlot<S> {
    type Param = SRes<Self>;

    fn get_state<'a>(param: &'a SystemParamItem<Self::Param>) -> Option<&'a S> {
        param.0.as_ref()
    }
}

impl<S: RawState> SetStateStorage<S> for StateSlot<S> {
    type Param = SResMut<Self>;

    fn get_state_from_mut<'s>(param: &'s SystemParamItem<Self::Param>) -> Option<&'s S> {
        param.0.as_ref()
    }

    fn get_state_mut<'s>(param: &'s mut SystemParamItem<Self::Param>) -> Option<&'s mut S> {
        param.0.as_mut()
    }

    fn set_state<'s>(param: &'s mut SystemParamItem<Self::Param>, state: Option<S>) {
        param.0 = state;
    }
}

#[cfg(feature = "bevy_app")]
impl<S: RawState> crate::app::AddStateStorage<S> for StateSlot<S> {
    fn add_state_storage(app: &mut bevy_app::App, state: Option<S>) {
        app.insert_resource(Self(state));
    }
}

impl<S: RawState> Default for StateSlot<S> {
    fn default() -> Self {
        Self::disabled()
    }
}

impl<S: RawState> StateSlot<S> {
    pub fn disabled() -> Self {
        Self(None)
    }

    pub fn enabled(value: S) -> Self {
        Self(Some(value))
    }

    pub fn get(&self) -> Option<&S> {
        self.0.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut S> {
        self.0.as_mut()
    }

    pub fn unwrap(&self) -> &S {
        self.get().unwrap()
    }

    pub fn unwrap_mut(&mut self) -> &mut S {
        self.get_mut().unwrap()
    }

    pub fn is_disabled(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_enabled(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_in<P: StatePattern<S>>(&self, pattern: &P) -> bool {
        matches!(self.get(), Some(x) if pattern.matches(x))
    }

    pub fn disable(&mut self) {
        self.0 = None;
    }

    // Enter the given state if disabled.
    pub fn enable(&mut self, value: S) -> &mut S {
        self.0.get_or_insert(value)
    }

    // Toggle between the given state and disabled.
    pub fn toggle(&mut self, value: S) {
        if self.is_enabled() {
            self.disable();
        } else {
            self.enter(value);
        }
    }

    pub fn enter(&mut self, value: S) -> &mut S {
        self.0.insert(value)
    }
}