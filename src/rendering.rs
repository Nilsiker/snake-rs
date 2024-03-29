use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

pub const TERMINAL_SIZE: [i32; 2] = [30, 30];

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_terminal);
    }
}

fn setup_terminal(mut commands: Commands) {
    let terminal = Terminal::new(TERMINAL_SIZE).with_border(Border::single_line());

    commands.spawn((TerminalBundle::from(terminal), AutoCamera));
}
