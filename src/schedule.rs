use std::{convert::Infallible, fmt::Debug, hash::Hash, marker::PhantomData};

use bevy_ecs::{
    event::Event,
    schedule::{IntoSystemConfigs, IntoSystemSetConfigs, Schedule, ScheduleLabel, SystemSet},
};

use crate::{
    state::State,
    systems::{
        apply_flush_state, clear_flush_state, flush_state, send_transition_event, state_will_flush,
        state_will_mutate, state_would_be_entering, state_would_be_exiting,
    },
};

#[derive(ScheduleLabel, Clone, Hash, PartialEq, Eq, Debug)]
pub struct PreStateTransition;

impl PreStateTransition {
    pub fn register_state<S: State>(schedule: &mut Schedule) {
        // TODO: Make this opt-out
        schedule.add_systems(flush_state::<S>.run_if(state_will_mutate::<S>));
    }
}

#[derive(ScheduleLabel, Clone, Hash, PartialEq, Eq, Debug)]
pub struct StateTransition;

impl StateTransition {
    // TODO: Configure the declared state dependencies
    pub fn register_state<S: State>(schedule: &mut Schedule) {
        schedule.configure_sets((
            OnTrans::<S>::Apply.run_if(state_will_flush::<S>),
            (
                OnTrans::<S>::Exit.run_if(state_would_be_exiting::<S>),
                OnTrans::<S>::Enter.run_if(state_would_be_entering::<S>),
            )
                .chain()
                .in_set(OnTrans::<S>::Apply),
        ));
    }
}

#[derive(ScheduleLabel, Clone, Hash, PartialEq, Eq, Debug)]
pub struct PostStateTransition;

impl PostStateTransition {
    pub fn register_state<S: State>(schedule: &mut Schedule) {
        // TODO: Make send_transition_event opt-out
        schedule.add_systems(
            ((
                (send_transition_event::<S>, apply_flush_state::<S>).chain(),
                clear_flush_state::<S>,
            ),)
                .run_if(state_will_flush::<S>),
        );
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Default)]
pub enum OnTrans<S> {
    #[default]
    Apply,
    Exit,
    Enter,
    _PhantomData(PhantomData<S>, Infallible),
}

impl<S> Hash for OnTrans<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl<S> Debug for OnTrans<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Apply => write!(f, "Any"),
            Self::Exit => write!(f, "Exit"),
            Self::Enter => write!(f, "Enter"),
            _ => unreachable!(),
        }
    }
}

#[derive(Event)]
pub struct StateTransitionEvent<S: State> {
    pub before: Option<S>,
    pub after: Option<S>,
}
