use std::collections::VecDeque;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::{game::in_playing_state, rendering::TERMINAL_SIZE, tick::TickEvent};

#[derive(Component)]
pub struct Snake {
    size: usize,
    tiles_occupied: VecDeque<IVec2>,
}

impl Default for Snake {
    fn default() -> Self {
        let start_tile: IVec2 = (TERMINAL_SIZE.map(|e| e / 2)).into();
        Self {
            size: 1,
            tiles_occupied: VecDeque::from(vec![start_tile]),
        }
    }
}

impl Snake {
    pub fn tiles(&self) -> &VecDeque<IVec2> {
        &self.tiles_occupied
    }
}

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&Direction> for IVec2 {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => IVec2::new(0, 1),
            Direction::Down => IVec2::new(0, -1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }
}

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake).add_systems(
            Update,
            (move_snake, change_direction).run_if(in_playing_state),
        );
    }
}

fn spawn_snake(mut commands: Commands) {
    let pos = IVec2::new(TERMINAL_SIZE[0] / 2, TERMINAL_SIZE[1] / 2);
    commands.spawn((Snake::default(), Position(pos), Direction::Up));
}

fn move_snake(
    mut snake: Query<(&mut Snake, &mut Position, &Direction), With<Snake>>,
    mut ticks: EventReader<TickEvent>,
) {
    for _ in ticks.read() {
        let (mut snake, mut position, direction) = snake.single_mut();
        position.0 += IVec2::from(direction);

        snake.tiles_occupied.push_front(position.0);
        if snake.tiles_occupied.len() > snake.size {
            snake.tiles_occupied.pop_back();
        }
    }
}

fn change_direction(
    mut events: EventReader<KeyboardInput>,
    mut snake_direction: Query<&mut Direction, With<Snake>>,
) {
    for event in events.read() {
        if event.state == ButtonState::Pressed {
            let mut snake_direction = snake_direction.single_mut();
            match event.key_code {
                KeyCode::KeyW if !matches!(*snake_direction, Direction::Down) => {
                    *snake_direction = Direction::Up;
                }
                KeyCode::KeyS if !matches!(*snake_direction, Direction::Up) => {
                    *snake_direction = Direction::Down;
                }
                KeyCode::KeyA if !matches!(*snake_direction, Direction::Right) => {
                    *snake_direction = Direction::Left;
                }
                KeyCode::KeyD if !matches!(*snake_direction, Direction::Left) => {
                    *snake_direction = Direction::Right;
                }
                _ => (),
            };
        }
    }
}
