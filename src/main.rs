mod food;
mod game;
mod glyphs;
mod input;
mod rendering;
mod score;
mod snake;
mod tick;

use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;
use food::FoodPlugin;
use game::GamePlugin;
use input::InputPlugin;
use rendering::RenderPlugin;
use score::ScorePlugin;
use snake::SnakePlugin;
use tick::TickPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TerminalPlugin,
            RenderPlugin,
            GamePlugin,
            SnakePlugin,
            InputPlugin,
            TickPlugin,
            FoodPlugin,
            ScorePlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
