use bevy::prelude::*;

pub const GLYPH_FOOD: char = '■';
pub const SNAKE_GLYPH: char = '█';

#[derive(Component)]
pub struct Glyph(pub char, pub Color);
