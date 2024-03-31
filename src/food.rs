use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    board::Board,
    game::GameState,
    glyphs,
    snake::{Glyph, Positions, Snake},
};

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConsumedEvent>()
            .add_systems(Update, spawn.run_if(in_state(GameState::Playing)))
            .add_systems(Update, respawn)
            .add_systems(OnEnter(GameState::GameOver), despawn_all);
    }
}

#[derive(Component)]
pub struct Food;

#[derive(Event)]
pub struct ConsumedEvent(pub Entity);

fn spawn(
    mut commands: Commands,
    foods: Query<&Food>,
    board: Res<Board>,
    positions: Query<&Positions, With<Snake>>,
) {
    let positions = positions.single();
    if foods.is_empty() {
        let mut rand = rand::thread_rng();

        let pos = board
            .tiles_not_in(&positions.0)
            .choose(&mut rand)
            .expect("at least one free space");
        commands.spawn((Food, Glyph(glyphs::FOOD, Color::RED), Positions(vec![*pos])));
    }
}

fn respawn(mut commands: Commands, mut events: EventReader<ConsumedEvent>) {
    for event in events.read() {
        commands.entity(event.0).despawn();
    }
}

fn despawn_all(mut commands: Commands, query: Query<Entity, With<Food>>) {
    query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
}
