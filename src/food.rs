use bevy::prelude::*;

use crate::{game::GameState, rendering::TERMINAL_SIZE, snake::Position};

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

fn spawn(mut commands: Commands, foods: Query<&Food>) {
    if foods.is_empty() {
        commands.spawn((Food, Position(get_random_position())));
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

fn get_random_position() -> IVec2 {
    use rand::prelude::*;

    let mut rand = rand::thread_rng();
    let random_pos = [
        rand.gen_range(0..TERMINAL_SIZE[0]),
        rand.gen_range(0..TERMINAL_SIZE[1]),
    ];

    random_pos.into()
}
