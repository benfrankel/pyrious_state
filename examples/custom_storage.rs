// Swap out or define your own state storage type.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ecs::system::lifetimeless::{SRes, SResMut};
use pyri_state::{
    extra::app::AddStateStorage,
    prelude::*,
    storage::{StateStorage, StateStorageMut},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StatePlugin))
        .insert_resource(StateDebugSettings {
            log_flush: true,
            ..default()
        })
        .init_state::<MyBufferedState>()
        .init_state::<MyStackedState>()
        .insert_state(StateSwap([
            Some(MySwappedState::X),
            Some(MySwappedState::Y),
        ]))
        .add_systems(
            Update,
            (
                MyStackedState::A
                    .push()
                    .run_if(input_just_pressed(KeyCode::KeyA)),
                MyStackedState::B
                    .push()
                    .run_if(input_just_pressed(KeyCode::KeyB)),
                MyStackedState::pop.run_if(input_just_pressed(KeyCode::Escape)),
                MySwappedState::swap.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .run();
}

#[derive(State, Clone, PartialEq, Eq, Default)]
// The default storage is `StateBuffer<Self>`, which is a newtyped `Option<Self>`.
//#[state(storage(StateBuffer<Self>))]
struct MyBufferedState;

#[derive(State, Clone, PartialEq, Eq, Debug, Default)]
// You can easily swap in a `StateStack<Self>` instead, for example.
#[state(log_flush, storage(StateStack<Self>))]
enum MyStackedState {
    #[default]
    A,
    B,
}

// You can define your own fully custom storage type:
#[derive(Resource)]
pub struct StateSwap<S: State>([Option<S>; 2]);

// Impl `StateStorage` to mark your type as a valid state storage type.
impl<S: State> StateStorage for StateSwap<S> {
    type State = S;

    type Param = SRes<Self>;

    // This allows `NextStateRef<S>` and `StateRef<S>` to interface with your storage type.
    fn get_state<'s>(param: &'s bevy_ecs::system::SystemParamItem<Self::Param>) -> Option<&'s S> {
        param.0[0].as_ref()
    }
}

// Impl `StateStorageMut` to support setting the next state through your storage type.
impl<S: State> StateStorageMut for StateSwap<S> {
    type ParamMut = SResMut<Self>;

    fn get_state_from_mut<'s>(
        param: &'s bevy_ecs::system::SystemParamItem<Self::ParamMut>,
    ) -> Option<&'s S> {
        param.0[0].as_ref()
    }

    // This allows `NextStateMut<S>` and `StateMut<S>` to interface with your storage type,
    fn get_state_mut<'s>(
        param: &'s mut bevy_ecs::system::SystemParamItem<Self::ParamMut>,
    ) -> Option<&'s mut S> {
        param.0[0].as_mut()
    }

    fn set_state(param: &mut bevy_ecs::system::SystemParamItem<Self::ParamMut>, state: Option<S>) {
        param.0[0] = state;
    }
}

// Impl `AddStateStorage` to support `app.add_state`, `init_state`, and `insert_state`.
impl<S: State> AddStateStorage for StateSwap<S> {
    fn add_state_storage(app: &mut bevy_app::App, storage: Option<Self>) {
        app.insert_resource(storage.unwrap_or_else(|| StateSwap([None, None])));
    }
}

// Define a custom extension trait to attach extra systems and run conditions to
// state types using your storage type.
pub trait StateSwapMut: State {
    fn swap(mut swap: ResMut<StateSwap<Self>>) {
        let [left, right] = &mut swap.0;
        std::mem::swap(left, right);
    }
}

// Blanket impl the trait.
impl<S: State<Storage = StateSwap<S>>> StateSwapMut for S {}

#[derive(State, Clone, PartialEq, Eq, Debug)]
// Now you can use `StateSwap<Self>` as a first-class custom storage type!
#[state(log_flush, storage(StateSwap<Self>))]
enum MySwappedState {
    X,
    Y,
}
