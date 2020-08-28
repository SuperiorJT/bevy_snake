use bevy::prelude::*;
use crate::plugins::game_state::{res, events};

/// System that runs when the [PreGamePhase] is active. 
///
/// This will tick the phase's timer and fire events for different lifecycle states.
/// ## Events
/// - [PreGameStartEvent]
/// - [PreGameEndEvent]
pub fn pre_game_system(
    time: Res<Time>,
    mut start_events: ResMut<Events<events::PreGameStartEvent>>,
    mut end_events: ResMut<Events<events::PreGameEndEvent>>,
    mut pre_game_state: ResMut<res::PreGamePhase>
) {
    if !pre_game_state.active {
        return;
    }

    let timer = &mut pre_game_state.timer;

    if timer.elapsed == 0.0 && time.delta_seconds != 0.0 {
        start_events.send(events::PreGameStartEvent);
    }

    timer.tick(time.delta_seconds);

    if timer.finished {
        timer.reset();
        pre_game_state.active = false;
        end_events.send(events::PreGameEndEvent);
    }
}

/// System that handles the active game phase when transitioning from [PreGamePhase] to [RunningGamePhase]
pub fn pre_to_run_transition_system(
    mut state: ResMut<res::PreGameEndListenerState>,
    pre_game_end_events: Res<Events<events::PreGameEndEvent>>,
    mut running_phase: ResMut<res::RunningGamePhase>,
    mut running_game_start_events: ResMut<Events<events::RunningGameStartEvent>>
) {
    for _ in state.event_reader.iter(&pre_game_end_events) {
        running_phase.active = true;
        running_game_start_events.send(events::RunningGameStartEvent);
    }
}

pub fn run_to_post_transition_system(
    mut state: ResMut<res::RunningGameEndListenerState>,
    run_end_events: Res<Events<events::RunningGameEndEvent>>,
    mut post_game_phase: ResMut<res::PostGamePhase>,
    mut running_game_phase: ResMut<res::RunningGamePhase>
) {
    for _ in state.event_reader.iter(&run_end_events) {
        running_game_phase.active = false;
        post_game_phase.active = true;
    }
}

/// System that runs when the [PreGamePhase] is active. 
///
/// This will tick the phase's timer and fire events for different lifecycle states.
/// ## Events
/// - [PostGameStartEvent]
/// - [PostGameEndEvent]
pub fn post_game_system(
    time: Res<Time>,
    mut start_events: ResMut<Events<events::PostGameStartEvent>>,
    mut end_events: ResMut<Events<events::PostGameEndEvent>>,
    mut post_game_state: ResMut<res::PostGamePhase>
) {
    if !post_game_state.active {
        return;
    }

    let timer = &mut post_game_state.timer;

    if timer.elapsed == 0.0 && time.delta_seconds != 0.0 {
        start_events.send(events::PostGameStartEvent);
    }

    timer.tick(time.delta_seconds);

    if timer.finished {
        timer.reset();
        post_game_state.active = false;
        end_events.send(events::PostGameEndEvent);
    }
}

pub fn post_to_pre_transition_system(
    mut state: ResMut<res::PostGameEndListenerState>,
    post_game_end_events: Res<Events<events::PostGameEndEvent>>,
    mut pre_game_phase: ResMut<res::PreGamePhase>
) {
    for _ in state.event_reader.iter(&post_game_end_events) {
        pre_game_phase.active = true;
    }
}