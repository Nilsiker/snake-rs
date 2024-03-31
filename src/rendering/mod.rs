pub mod data;
mod systems;

use bevy::prelude::*;

use crate::core::data::GameState;

use self::systems::{
    clear_terminal, render_game_over, render_glyphs, render_start_screen, setup_terminal,
};

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_terminal)
            .add_systems(
                PreUpdate,
                clear_terminal.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, render_glyphs.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Start), render_start_screen)
            .add_systems(OnEnter(GameState::GameOver), render_game_over);
    }
}
