#![allow(clippy::type_complexity)]

mod core;
mod food;
mod input;
mod rendering;
mod score;
mod snake;

use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .add_plugins(snake_rs::SnakePlugins)
        .run();
}
