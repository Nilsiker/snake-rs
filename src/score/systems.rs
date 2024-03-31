use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::core::data::SnakeEvent;

use super::data::*;

pub fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

pub fn increase_score(mut score: ResMut<Score>, mut reader: EventReader<SnakeEvent>) {
    if reader.read().any(|ev| matches!(*ev, SnakeEvent::Crashed)) {
        score.0 += 10;
    }
}

pub fn render_score(mut terminal: Query<&mut Terminal>, score: Res<Score>) {
    let mut terminal = terminal.single_mut();

    terminal
        .border_mut()
        .unwrap()
        .set_title_string(format!("Score: {}", score.0));
}
