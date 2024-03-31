pub mod data;
mod systems;

use bevy::prelude::*;

use crate::core::data::GameState;

use self::systems::{despawn_all, handle_consumed, spawn};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn.run_if(in_state(GameState::Playing)))
            .add_systems(Update, handle_consumed)
            .add_systems(OnEnter(GameState::GameOver), despawn_all);
    }
}
