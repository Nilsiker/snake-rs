use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::{food::ConsumedEvent, game::GameState};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(OnEnter(GameState::Playing), reset_score)
            .add_systems(
                Update,
                (increase_score, render_score).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
pub struct Score(pub usize);

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

fn increase_score(mut score: ResMut<Score>, mut events: EventReader<ConsumedEvent>) {
    for _ in events.read() {
        score.0 += 10;
    }
}

fn render_score(mut terminal: Query<&mut Terminal>, score: Res<Score>) {
    let mut terminal = terminal.single_mut();

    terminal
        .border_mut()
        .unwrap()
        .set_title_string(format!("Score: {}", score.0));
}
