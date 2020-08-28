use bevy::prelude::*;

pub mod res;
pub mod sys;
pub mod events;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_event::<events::PreGameStartEvent>()
        .add_event::<events::PreGameEndEvent>()
        .add_event::<events::RunningGameStartEvent>()
        .add_event::<events::RunningGameEndEvent>()
        .add_event::<events::PostGameStartEvent>()
        .add_event::<events::PostGameEndEvent>()
        .add_resource(res::PreGameStartListenerState::default())
        .add_resource(res::PreGameEndListenerState::default())
        .add_resource(res::RunningGameStartListenerState::default())
        .add_resource(res::RunningGameEndListenerState::default())
        .add_resource(res::PostGameStartListenerState::default())
        .add_resource(res::PostGameEndListenerState::default())
        .add_resource(res::PreGamePhase::new(true))
        .add_resource(res::RunningGamePhase::new(false))
        .add_resource(res::PostGamePhase::new(false))
        .add_system(sys::pre_game_system.system())
        .add_system(sys::post_game_system.system())
        .add_system(sys::pre_to_run_transition_system.system())
        .add_system(sys::run_to_post_transition_system.system())
        .add_system(sys::post_to_pre_transition_system.system());
    }
}