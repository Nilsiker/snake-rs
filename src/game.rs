use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::{
    rendering::TERMINAL_SIZE,
    snake::{Position, Snake},
};

#[derive(Resource)]
pub enum State {
    Start,
    Playing,
    GameOver,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(State::Start)
            .add_systems(Update, check_start.run_if(not_in_playing_state))
            .add_systems(PostUpdate, check_game_over.run_if(in_playing_state));
    }
}

fn check_start(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<State>) {
    if input.just_pressed(KeyCode::Space) {
        *state = State::Playing;
        println!("Start!");
    }
}

fn check_game_over(
    // TODO event system for position change?
    snake: Query<&Position, (With<Snake>, Changed<Position>)>,
    terminal: Query<&Terminal>,
    mut state: ResMut<State>,
) {
    let Ok(position) = snake.get_single() else {
        return;
    };

    let terminal = terminal.single();

    let bounds = terminal.bounds().translated(TERMINAL_SIZE.map(|e| e / 2));

    if !bounds.contains(position.0) {
        *state = State::GameOver;
        println!("Inside bounds.");
    }
}

pub fn in_playing_state(state: Res<State>) -> bool {
    matches!(*state, State::Playing)
}

// TODO find way to negate run_if criteria?
pub fn not_in_playing_state(state: Res<State>) -> bool {
    !matches!(*state, State::Playing)
}
