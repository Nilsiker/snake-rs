use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::{
    food::{ConsumedEvent, Food},
    game::{CrashEvent, GameState},
    glyphs::SNAKE,
    rendering::TERMINAL_SIZE,
    tick::TickEvent,
};

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct Size(pub usize);

#[derive(Component)]
pub struct Glyph(pub char, pub Color);

#[derive(Component)]
pub struct Positions(pub Vec<IVec2>);

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
    commands.spawn((
        Snake,
        Size(1),
        Positions(vec![pos]),
        Glyph(SNAKE, Color::GREEN),
        Direction::Up,
    ));
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
    mut snake: Query<(&Size, &mut Positions, &Direction), With<Snake>>,
    mut ticks: EventReader<TickEvent>,
    mut crashes: EventWriter<CrashEvent>,
) {
    for _ in ticks.read() {
        let (Size(size), mut positions, direction) = snake.single_mut();
        let new_position = positions.0[0] + IVec2::from(direction);

        if positions.0.contains(&new_position) {
            crashes.send(CrashEvent);
        }

        positions.0.insert(0, new_position);
        if positions.0.len() > *size {
            positions.0.pop();
        }

        println!("{:#?}", positions.0);
    }
}

fn check_for_food(
    mut snake: Query<(&mut Size, &Positions), (Changed<Positions>, With<Snake>)>,
    foods: Query<(&Positions, Entity), With<Food>>,
    mut events: EventWriter<ConsumedEvent>,
) {
    let Ok((mut size, pos)) = snake.get_single_mut() else {
        return;
    };

    let current_size = size.0;

    foods
        .iter()
        .filter(|(food_pos, _)| food_pos.0[0] == pos.0[0])
        .for_each(|(_, entity)| {
            events.send(ConsumedEvent(entity));
            size.0 += 2.max(current_size / 3).min(10);
        })
}

fn check_snake_in_bounds(
    snake: Query<&Positions, (With<Snake>, Changed<Positions>)>,
    terminal: Query<&Terminal>,
    mut crashes: EventWriter<CrashEvent>,
) {
    let Ok(position) = snake.get_single() else {
        return;
    };

    let terminal = terminal.single();
    let bounds = terminal.bounds().translated(TERMINAL_SIZE.map(|e| e / 2));

    if !bounds.contains(position.0[0]) {
        crashes.send(CrashEvent);
    }
}
