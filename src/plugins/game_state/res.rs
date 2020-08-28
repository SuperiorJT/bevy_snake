use bevy::prelude::*;
use crate::plugins::game_state::events;

pub const PRE_GAME_DURATION: f32 = 3.0;
pub const POST_GAME_DURATION: f32 = 4.0;

pub struct PreGamePhase {
    pub active: bool,
    pub timer: Timer
}
impl PreGamePhase {
    pub fn new(active: bool) -> Self {
        PreGamePhase {
            active,
            timer: Timer::from_seconds(PRE_GAME_DURATION, false)
        }
    }
}

#[derive(Default)]
pub struct PreGameStartListenerState {
    pub event_reader: EventReader<events::PreGameStartEvent>
}

#[derive(Default)]
pub struct PreGameEndListenerState {
    pub event_reader: EventReader<events::PreGameEndEvent>
}

pub struct RunningGamePhase {
    pub active: bool
}
impl RunningGamePhase {
    pub fn new(active: bool) -> Self {
        RunningGamePhase {
            active
        }
    }
}

#[derive(Default)]
pub struct RunningGameStartListenerState {
    pub event_reader: EventReader<events::RunningGameStartEvent>
}

#[derive(Default)]
pub struct RunningGameEndListenerState {
    pub event_reader: EventReader<events::RunningGameEndEvent>
}

pub struct PostGamePhase {
    pub active: bool,
    pub timer: Timer
}
impl PostGamePhase {
    pub fn new(active: bool) -> Self {
        PostGamePhase {
            active,
            timer: Timer::from_seconds(POST_GAME_DURATION, false)
        }
    }
}

#[derive(Default)]
pub struct PostGameStartListenerState {
    pub event_reader: EventReader<events::PostGameStartEvent>
}

#[derive(Default)]
pub struct PostGameEndListenerState {
    pub event_reader: EventReader<events::PostGameEndEvent>
}