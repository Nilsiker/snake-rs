pub mod data;
mod systems;

use bevy::prelude::*;

use self::systems::*;
use crate::core::data::GameState;

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn)
            .add_systems(OnEnter(GameState::GameOver), despawn)
            .add_systems(
                PreUpdate,
                update_direction.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (move_snake, check_for_food, check_snake_in_bounds)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
