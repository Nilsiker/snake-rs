mod rendering;
mod snake;
mod tick;

use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;
use rendering::RenderPlugin;
use snake::SnakePlugin;
use tick::TickPlugin;

fn main() {
    let tick_ms = match std::env::var("TICK") {
        Ok(value) => value.parse().expect("parseable usize"),
        Err(_) => 120,
    };
    App::new()
        .add_plugins((
            DefaultPlugins,
            TerminalPlugin,
            RenderPlugin,
            SnakePlugin,
            TickPlugin::new(tick_ms),
        ))
        .run();
}
