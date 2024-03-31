use bevy::prelude::*;
use rand::seq::{IteratorRandom, SliceRandom};

use crate::{
    core::data::{Board, Positions, SnakeEvent},
    rendering::data::{Glyph, GLYPH_FOOD},
    snake::data::Snake,
};

use super::data::Food;

pub fn spawn(
    mut commands: Commands,
    foods: Query<&Food>,
    board: Res<Board>,
    positions: Query<&Positions, With<Snake>>,
) {
    if foods.is_empty() {
        let mut rand = rand::thread_rng();
        let position = match positions.get_single() {
            Ok(positions) => board
                .tiles_not_in(&positions.0)
                .choose(&mut rand)
                .expect("at least one free space"),
            Err(_) => board
                .tiles()
                .choose(&mut rand)
                .expect("at least one free space"),
        };

        commands.spawn((
            Food,
            Glyph(GLYPH_FOOD, Color::RED),
            Positions(vec![*position]),
        ));
    }
}

pub fn handle_consumed(mut commands: Commands, mut reader: EventReader<SnakeEvent>) {
    for event in reader.read() {
        if let SnakeEvent::FoodConsumed(entity) = event {
            commands.entity(*entity).despawn();
        }
    }
}

pub fn despawn_all(mut commands: Commands, query: Query<Entity, With<Food>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
