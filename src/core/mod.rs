pub mod data;
mod systems;

use bevy::prelude::*;

use self::{
    data::{Board, GameState, SnakeEvent, TickEvent, TickTimer},
    systems::{check_start, increase_tick_rate, register_game_over, set_starting_tick_rate, tick},
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Start)
            .add_event::<SnakeEvent>()
            .add_event::<TickEvent>()
            .insert_resource(Board::new(30))
            .insert_resource(TickTimer(Timer::from_seconds(0.0, TimerMode::Repeating)))
            .add_systems(OnEnter(GameState::Playing), set_starting_tick_rate)
            .add_systems(Update, (increase_tick_rate, tick))
            .add_systems(Update, check_start.run_if(in_state(GameState::Start)))
            .add_systems(Update, check_start.run_if(in_state(GameState::GameOver)))
            .add_systems(PostUpdate, register_game_over)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}
