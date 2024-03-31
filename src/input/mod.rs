mod systems;

use bevy::prelude::*;

use self::systems::*;
use crate::core::data::GameState;
use crate::core::data::MoveDirection;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveDirection::Up)
            .add_systems(OnEnter(GameState::Playing), reset_direction)
            .add_systems(PreUpdate, input.run_if(in_state(GameState::Playing)));
    }
}
