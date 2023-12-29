pub mod lod_group;
pub mod resolver;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CurrentLod(u8);

#[derive(Default, Resource)]
pub struct LodSettings {
    pub bias: i8,
}
