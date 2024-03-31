use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::{
    game::GameState,
    snake::{Direction, Snake},
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Direction::Up)
            .add_systems(PreUpdate, input.run_if(in_state(GameState::Playing)));
    }
}

fn input(
    mut events: EventReader<KeyboardInput>,
    mut direction: ResMut<Direction>,
    snake_direction: Query<&Direction, With<Snake>>,
) {
    for event in events.read() {
        if event.state == ButtonState::Pressed {
            let snake_direction = snake_direction.single();
            match event.key_code {
                KeyCode::KeyW if !matches!(*snake_direction, Direction::Down) => {
                    *direction = Direction::Up;
                }
                KeyCode::KeyS if !matches!(*snake_direction, Direction::Up) => {
                    *direction = Direction::Down;
                }
                KeyCode::KeyA if !matches!(*snake_direction, Direction::Right) => {
                    *direction = Direction::Left;
                }
                KeyCode::KeyD if !matches!(*snake_direction, Direction::Left) => {
                    *direction = Direction::Right;
                }
                _ => (),
            };
        }
    }
}
