use bevy::prelude::*;

use crate::rendering::TERMINAL_SIZE;

#[derive(Resource)]
pub struct Board(Vec<IVec2>);

impl Board {
    pub fn new() -> Self {
        let mut tiles = vec![];
        for x in 0..TERMINAL_SIZE[0] {
            for y in 0..TERMINAL_SIZE[1] {
                tiles.push([x, y].into());
            }
        }
        Self(tiles)
    }

    pub fn tiles_not_in<'a>(&'a self, other: &'a [IVec2]) -> impl Iterator<Item = &'a IVec2> {
        let tiles = &self.0;

        tiles.iter().filter(|tile| !other.contains(tile))
    }
}
