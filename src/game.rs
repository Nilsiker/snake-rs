use bevy::prelude::*;

use crate::board::Board;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    Start,
    Playing,
    GameOver,
}
#[derive(Event)]
pub struct CrashEvent;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Start)
            .insert_resource(Board::new())
            .add_event::<CrashEvent>()
            .add_systems(Update, check_start.run_if(in_state(GameState::Start)))
            .add_systems(Update, check_start.run_if(in_state(GameState::GameOver)))
            .add_systems(PostUpdate, register_game_over);
    }
}

fn check_start(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
        println!("Start!");
    }
}

fn register_game_over(
    mut events: EventReader<CrashEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    if !events.is_empty() {
        events.clear();
        state.set(GameState::GameOver);
    }
}
