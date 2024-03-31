use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

use crate::{
    food::Food,
    game::{in_playing_state, not_in_playing_state},
    snake::{Position, Snake},
};

pub const TERMINAL_SIZE: [i32; 2] = [30, 30];

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_terminal)
            .add_systems(PreUpdate, clear_terminal)
            .add_systems(Update, (render_food, render_snake).run_if(in_playing_state))
            .add_systems(Update, render_start_screen.run_if(not_in_playing_state));
    }
}

fn setup_terminal(mut commands: Commands) {
    let terminal = Terminal::new(TERMINAL_SIZE).with_border(Border::single_line());
    commands.spawn((TerminalBundle::from(terminal), AutoCamera));
}

fn clear_terminal(mut terminal: Query<&mut Terminal>) {
    let mut term = terminal.single_mut();

    term.clear();
}

fn render_snake(mut terminal: Query<&mut Terminal>, snake: Query<&Snake>) {
    let snake = snake.single();
    let mut terminal = terminal.single_mut();
    let mut bounds = terminal.bounds();
    bounds = bounds.translated(TERMINAL_SIZE.map(|e| e / 2));

    snake
        .tiles()
        .iter()
        .filter(|pos| bounds.contains(**pos))
        .for_each(|pos| terminal.put_char(*pos, '█'));
}

fn render_food(query: Query<&Position, With<Food>>, mut terminal: Query<&mut Terminal>) {
    let mut terminal = terminal.single_mut();

    query.iter().for_each(|food| terminal.put_char(food.0, '*'));
}

fn render_start_screen(mut terminal: Query<&mut Terminal>) {
    let mut terminal = terminal.single_mut();
    let title = "SNAKE-RS".to_string();
    let prompt = "Press start to play".to_string();

    let title_offset: i32 = title.len() as i32 / 2;
    let prompt_offset: i32 = prompt.len() as i32 / 2;

    let mut title_pos = TERMINAL_SIZE.map(|e| e / 2);
    title_pos[1] += 1;
    title_pos[0] -= title_offset;
    let mut prompt_pos = TERMINAL_SIZE.map(|e| e / 2);
    prompt_pos[1] -= 1;
    prompt_pos[0] -= prompt_offset;

    terminal.put_string(title_pos, title);
    terminal.put_string(prompt_pos, prompt);
}
