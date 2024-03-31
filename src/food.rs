use bevy::prelude::*;

use crate::{game::State, rendering::TERMINAL_SIZE, snake::Position};

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, spawn_food.run_if(should_spawn_food));
    }
}

#[derive(Component)]
pub struct Food;

fn spawn_food(mut commands: Commands) {
    println!("spawning food");
    commands.spawn((Food, Position(get_random_position())));
}

fn should_spawn_food(query: Query<&Food>, state: Res<State>) -> bool {
    query.is_empty() && matches!(*state, State::Playing)
}

fn get_random_position() -> IVec2 {
    TERMINAL_SIZE.map(|e| e / 3).into()
}
