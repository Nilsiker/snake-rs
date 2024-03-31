use std::time::Duration;

use bevy::prelude::*;

use crate::core::GameState;

use super::data::{SnakeEvent, TickEvent, TickTimer};

pub fn check_start(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
        println!("Start!");
    }
}

pub fn register_game_over(
    mut events: EventReader<SnakeEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    if events.read().any(|ev| matches!(*ev, SnakeEvent::Crashed)) {
        state.set(GameState::GameOver);
    }
}

pub fn set_starting_tick_rate(mut timer: ResMut<TickTimer>) {
    let ms = match std::env::var("TICK") {
        Ok(value) => value.parse().expect("parseable usize"),
        Err(_) => 150,
    };
    timer.0.set_duration(Duration::from_millis(ms));
}

pub fn tick(mut reader: EventWriter<TickEvent>, mut timer: ResMut<TickTimer>, time: Res<Time>) {
    if timer.0.finished() {
        timer.0.reset();
        reader.send(TickEvent);
    } else {
        timer.0.tick(time.delta());
    }
}

pub fn increase_tick_rate(mut timer: ResMut<TickTimer>, mut events: EventReader<SnakeEvent>) {
    if timer.0.duration().as_millis() < 80 {
        return;
    }

    for _ in events.read() {
        let current_duration = timer.0.duration();
        println!("{:#?}", current_duration);
        let new_duration = Duration::from_millis(current_duration.as_millis() as u64 - 10);
        timer.0.set_duration(new_duration);
    }
}
