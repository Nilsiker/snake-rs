use std::time::Duration;

use bevy::prelude::*;

use crate::{food::ConsumedEvent, game::GameState};

#[derive(Event)]
pub struct TickEvent;

#[derive(Resource)]
struct TickTimer(Timer);

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>()
            .insert_resource(TickTimer(Timer::from_seconds(0.0, TimerMode::Repeating)))
            .add_systems(OnEnter(GameState::Playing), set_starting_tick_rate)
            .add_systems(Update, (increase_tick_rate, tick));
    }
}

fn set_starting_tick_rate(mut timer: ResMut<TickTimer>) {
    let ms = match std::env::var("TICK") {
        Ok(value) => value.parse().expect("parseable usize"),
        Err(_) => 150,
    };
    timer.0.set_duration(Duration::from_millis(ms));
}

fn tick(mut tick_events: EventWriter<TickEvent>, mut timer: ResMut<TickTimer>, time: Res<Time>) {
    if timer.0.finished() {
        timer.0.reset();
        tick_events.send(TickEvent);
    } else {
        timer.0.tick(time.delta());
    }
}

fn increase_tick_rate(mut timer: ResMut<TickTimer>, mut events: EventReader<ConsumedEvent>) {
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
