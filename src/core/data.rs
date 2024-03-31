use bevy::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    Start,
    Playing,
    GameOver,
}

#[derive(Event)]
pub enum SnakeEvent {
    FoodConsumed(Entity),
    Crashed,
}

#[derive(Resource)]
pub struct Board {
    side: usize,
    tiles: Vec<IVec2>,
}

impl Board {
    pub fn new(side: usize) -> Self {
        let mut tiles = vec![];
        for x in 0..side {
            for y in 0..side {
                tiles.push([x as i32, y as i32].into());
            }
        }
        Self { side, tiles }
    }

    pub fn side(&self) -> usize {
        self.side
    }

    pub fn tiles_not_in<'a>(&'a self, other: &'a [IVec2]) -> impl Iterator<Item = &'a IVec2> {
        self.tiles.iter().filter(|tile| !other.contains(tile))
    }
}

#[derive(Component)]
pub struct Positions(pub Vec<IVec2>);

#[derive(Component, Resource, Clone)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<&MoveDirection> for IVec2 {
    fn from(value: &MoveDirection) -> Self {
        match value {
            MoveDirection::Up => IVec2::new(0, 1),
            MoveDirection::Down => IVec2::new(0, -1),
            MoveDirection::Left => IVec2::new(-1, 0),
            MoveDirection::Right => IVec2::new(1, 0),
        }
    }
}

#[derive(Event)]
pub struct TickEvent;

#[derive(Resource)]
pub(super) struct TickTimer(pub Timer);
