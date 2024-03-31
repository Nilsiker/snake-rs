use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::core::data::MoveDirection;
use crate::snake::data::Snake;

pub fn reset_direction(mut dir: ResMut<MoveDirection>) {
    *dir = MoveDirection::Up;
}

pub fn input(
    mut events: EventReader<KeyboardInput>,
    mut direction: ResMut<MoveDirection>,
    snake_direction: Query<&MoveDirection, With<Snake>>,
) {
    for event in events.read() {
        if event.state == ButtonState::Pressed {
            let snake_direction = snake_direction.single();
            match event.key_code {
                KeyCode::KeyW | KeyCode::ArrowUp
                    if !matches!(*snake_direction, MoveDirection::Down) =>
                {
                    *direction = MoveDirection::Up;
                }
                KeyCode::KeyS | KeyCode::ArrowDown
                    if !matches!(*snake_direction, MoveDirection::Up) =>
                {
                    *direction = MoveDirection::Down;
                }
                KeyCode::KeyA | KeyCode::ArrowLeft
                    if !matches!(*snake_direction, MoveDirection::Right) =>
                {
                    *direction = MoveDirection::Left;
                }
                KeyCode::KeyD | KeyCode::ArrowRight
                    if !matches!(*snake_direction, MoveDirection::Left) =>
                {
                    *direction = MoveDirection::Right;
                }
                _ => (),
            };
        }
    }
}
