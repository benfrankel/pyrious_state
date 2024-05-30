pub mod slot;
pub mod stack;

use bevy_ecs::system::{ReadOnlySystemParam, SystemParam, SystemParamItem};

use crate::state::{GetState, RawState, SetState};

// Marker trait for types that can be used as a state's storage.
pub trait StateStorage<S: RawState> {}

pub trait GetStateStorage<S: RawState> {
    type Param: ReadOnlySystemParam;

    fn get_state<'s>(param: &'s SystemParamItem<Self::Param>) -> Option<&'s S>;
}

// A state is `GetState` if its storage is `GetStateStorage`.
impl<S: RawState> GetState for S
where
    S::Storage: GetStateStorage<S>,
{
    type GetStorage = S::Storage;
}

pub trait SetStateStorage<S: RawState> {
    type Param: SystemParam;

    fn get_state_from_mut<'s>(param: &'s SystemParamItem<Self::Param>) -> Option<&'s S>;

    fn get_state_mut<'s>(param: &'s mut SystemParamItem<Self::Param>) -> Option<&'s mut S>;

    fn set_state(param: &mut SystemParamItem<Self::Param>, state: Option<S>);
}

// A state is `SetState` if its storage is `SetStateStorage`.
impl<S: RawState> SetState for S
where
    S::Storage: SetStateStorage<S>,
{
    type SetStorage = S::Storage;
}
