#![allow(clippy::type_complexity)]

mod core;
mod food;
mod input;
mod rendering;
mod score;
mod snake;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct SnakePlugins;
impl PluginGroup for SnakePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(core::CorePlugin)
            .add(rendering::RenderPlugin)
            .add(input::InputPlugin)
            .add(snake::SnakePlugin)
            .add(food::FoodPlugin)
            .add(score::ScorePlugin)
    }
}
