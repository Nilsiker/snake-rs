use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::{
    core::data::{Board, Positions, SnakeEvent, TickEvent},
    food::data::Food,
    rendering::data::{Glyph, SNAKE_GLYPH},
};

use super::data::{Size, Snake};
use crate::core::data::MoveDirection;

pub fn spawn(mut commands: Commands, board: Res<Board>) {
    let pos = IVec2::new(board.side() as i32 / 2, board.side() as i32 / 2);
    commands.spawn((
        Snake,
        Size(1),
        Positions(vec![pos]),
        Glyph(SNAKE_GLYPH, Color::GREEN),
        MoveDirection::Up,
    ));
}

pub fn despawn(mut commands: Commands, snake: Query<Entity, With<Snake>>) {
    let snake = snake.single();
    commands.entity(snake).despawn();
}

pub fn update_direction(
    mut reader: EventReader<TickEvent>,
    mut snake: Query<&mut MoveDirection, With<Snake>>,
    input_dir: Res<MoveDirection>,
) {
    for _ in reader.read() {
        let mut dir = snake.single_mut();
        *dir = input_dir.clone();
    }
}

pub fn move_snake(
    mut snake: Query<(&Size, &mut Positions, &MoveDirection), With<Snake>>,
    mut reader: EventReader<TickEvent>,
    mut writer: EventWriter<SnakeEvent>,
) {
    for _ in reader.read() {
        let (Size(size), mut positions, direction) = snake.single_mut();
        let new_position = positions.0[0] + IVec2::from(direction);

        if positions.0.contains(&new_position) {
            writer.send(SnakeEvent::Crashed);
        }

        positions.0.insert(0, new_position);
        if positions.0.len() > *size {
            positions.0.pop();
        }
    }
}

pub fn check_for_food(
    mut snake: Query<(&mut Size, &Positions), (Changed<Positions>, With<Snake>)>,
    foods: Query<(&Positions, Entity), With<Food>>,
    mut events: EventWriter<SnakeEvent>,
) {
    let Ok((mut size, pos)) = snake.get_single_mut() else {
        return;
    };

    let current_size = size.0;

    foods
        .iter()
        .filter(|(food_pos, _)| food_pos.0[0] == pos.0[0])
        .for_each(|(_, entity)| {
            events.send(SnakeEvent::FoodConsumed(entity));
            size.0 += 2.max(current_size / 3).min(10);
        })
}

pub fn check_snake_in_bounds(
    snake: Query<&Positions, (With<Snake>, Changed<Positions>)>,
    terminal: Query<&Terminal>,
    mut crashes: EventWriter<SnakeEvent>,
) {
    let Ok(position) = snake.get_single() else {
        return;
    };

    let terminal = terminal.single();
    let bounds = terminal
        .bounds()
        .translated([terminal.width() / 2, terminal.height() / 2]);

    if !bounds.contains(position.0[0]) {
        crashes.send(SnakeEvent::Crashed);
    }
}
