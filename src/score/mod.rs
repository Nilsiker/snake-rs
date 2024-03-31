pub mod data;
mod systems;

use bevy::prelude::*;

use crate::core::data::GameState;

use self::data::Score;
use self::systems::*;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(OnEnter(GameState::Playing), reset_score)
            .add_systems(
                Update,
                (increase_score, render_score).run_if(in_state(GameState::Playing)),
            );
    }
}
