use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::{
    core::data::{Board, Positions},
    score::data::Score,
};

use super::data::Glyph;

pub fn setup_terminal(mut commands: Commands, board: Res<Board>) {
    let terminal = Terminal::new([board.side(), board.side()]).with_border(Border::single_line());
    commands.spawn((TerminalBundle::from(terminal), AutoCamera));
}

pub fn clear_terminal(mut terminal: Query<&mut Terminal>) {
    let mut term = terminal.single_mut();

    term.clear();
}

pub fn render_glyphs(query: Query<(&Positions, &Glyph)>, mut terminal: Query<&mut Terminal>) {
    let mut terminal = terminal.single_mut();
    let mut bounds = terminal.bounds();
    bounds = bounds.translated([terminal.width() / 2, terminal.height() / 2]);
    for (Positions(positions), Glyph(glyph, color)) in &query {
        positions
            .iter()
            .filter(|pos| bounds.contains(**pos))
            .for_each(|pos| terminal.put_char(*pos, glyph.fg(*color)));
    }
}

pub fn render_start_screen(mut terminal: Query<&mut Terminal>) {
    let mut terminal = terminal.single_mut();
    let title = "SNAKE-RS".to_string();
    let prompt = "Press start to play".to_string();

    let title_offset: i32 = title.len() as i32 / 2;
    let prompt_offset: i32 = prompt.len() as i32 / 2;

    let mut title_pos = IVec2::new(terminal.width() as i32 / 2, terminal.height() as i32 / 2);
    title_pos[1] += 1;
    title_pos[0] -= title_offset;
    let mut prompt_pos = IVec2::new(terminal.width() as i32 / 2, terminal.height() as i32 / 2);
    prompt_pos[1] -= 1;
    prompt_pos[0] -= prompt_offset;

    terminal.put_string(title_pos, title);
    terminal.put_string(prompt_pos, prompt);
}

pub fn render_game_over(mut terminal: Query<&mut Terminal>, score: Res<Score>) {
    let mut terminal = terminal.single_mut();
    let score = score.0;

    let title = format!("YOUR SCORE: {score}");
    let prompt = "Press start to play again".to_string();

    let title_offset: i32 = title.len() as i32 / 2;
    let prompt_offset: i32 = prompt.len() as i32 / 2;

    let mut title_pos = IVec2::new(terminal.width() as i32 / 2, terminal.height() as i32 / 2);
    title_pos[1] += 1;
    title_pos[0] -= title_offset;
    let mut prompt_pos = IVec2::new(terminal.width() as i32 / 2, terminal.height() as i32 / 2);
    prompt_pos[1] -= 1;
    prompt_pos[0] -= prompt_offset;

    terminal.put_string(title_pos, title);
    terminal.put_string(prompt_pos, prompt);
}
