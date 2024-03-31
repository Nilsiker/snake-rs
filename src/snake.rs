use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::{
    food::{ConsumedEvent, Food},
    game::{CrashEvent, GameState},
    rendering::TERMINAL_SIZE,
    tick::TickEvent,
};

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

#[derive(Component, PartialEq, Eq)]
pub struct Position(pub IVec2);

#[derive(Component, Resource, Clone)]
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
        app.add_systems(OnEnter(GameState::Playing), spawn)
            .add_systems(OnEnter(GameState::GameOver), despawn)
            .add_systems(
                PreUpdate,
                update_direction.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (move_snake, check_for_food, check_snake_in_bounds)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn(mut commands: Commands) {
    let pos = IVec2::new(TERMINAL_SIZE[0] / 2, TERMINAL_SIZE[1] / 2);
    commands.spawn((Snake::default(), Position(pos), Direction::Up));
}

fn despawn(mut commands: Commands, snake: Query<Entity, With<Snake>>) {
    let snake = snake.single();
    commands.entity(snake).despawn();
}

fn update_direction(
    mut ticks: EventReader<TickEvent>,
    mut snake: Query<&mut Direction, With<Snake>>,
    input_dir: Res<Direction>,
) {
    for _ in ticks.read() {
        let mut dir = snake.single_mut();
        *dir = input_dir.clone();
    }
}

fn move_snake(
    mut snake: Query<(&mut Snake, &mut Position, &Direction), With<Snake>>,
    mut ticks: EventReader<TickEvent>,
    mut crashes: EventWriter<CrashEvent>,
) {
    for _ in ticks.read() {
        let (mut snake, mut position, direction) = snake.single_mut();
        position.0 += IVec2::from(direction);

        if snake.tiles_occupied.contains(&position.0) {
            crashes.send(CrashEvent);
        }

        snake.tiles_occupied.push_front(position.0);
        if snake.tiles_occupied.len() > snake.size {
            snake.tiles_occupied.pop_back();
        }
    }
}

fn check_for_food(
    mut snake: Query<(&mut Snake, &Position), Changed<Position>>,
    foods: Query<(&Position, Entity), With<Food>>,
    mut events: EventWriter<ConsumedEvent>,
) {
    let Ok((mut snake, pos)) = snake.get_single_mut() else {
        return;
    };

    foods
        .iter()
        .filter(|(food_pos, _)| food_pos.0 == pos.0)
        .for_each(|(_, entity)| {
            events.send(ConsumedEvent(entity));
            snake.size += 2.max(snake.size / 3).min(10);
        })
}

fn check_snake_in_bounds(
    snake: Query<&Position, (With<Snake>, Changed<Position>)>,
    terminal: Query<&Terminal>,
    mut crashes: EventWriter<CrashEvent>,
) {
    let Ok(position) = snake.get_single() else {
        return;
    };

    let terminal = terminal.single();
    let bounds = terminal.bounds().translated(TERMINAL_SIZE.map(|e| e / 2));

    if !bounds.contains(position.0) {
        crashes.send(CrashEvent);
    }
}
