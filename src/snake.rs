use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_ascii_terminal::Terminal;

use crate::{rendering::TERMINAL_SIZE, tick::TickEvent};

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct Position(IVec2);

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
        app.add_systems(Startup, spawn_snake)
            .add_systems(Update, (render_snake, move_snake, change_direction));
    }
}

fn spawn_snake(mut commands: Commands) {
    let pos = IVec2::new(TERMINAL_SIZE[0] / 2, TERMINAL_SIZE[1] / 2);
    commands.spawn((Snake, Position(pos), Direction::Up));
}

fn render_snake(mut terminal: Query<&mut Terminal>, snake: Query<&Position, With<Snake>>) {
    let snake_pos = snake.single();
    let mut terminal = terminal.single_mut();

    terminal.put_char(snake_pos.0, '█');
}

fn move_snake(
    mut snake: Query<(&mut Position, &Direction), With<Snake>>,
    mut ticks: EventReader<TickEvent>,
) {
    for _ in ticks.read() {
        let (mut position, direction) = snake.single_mut();
        position.0 += IVec2::from(direction);
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
                KeyCode::KeyW => *snake_direction = Direction::Up,
                KeyCode::KeyS => *snake_direction = Direction::Down,
                KeyCode::KeyA => *snake_direction = Direction::Left,
                KeyCode::KeyD => *snake_direction = Direction::Right,
                _ => (),
            };
        }
    }
}
