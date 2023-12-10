use bevy::prelude::*;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

pub mod components;
pub mod systems;

pub const CHUNK_HEIGHT_IN_BLOCKS: u16 = 16;
pub const CHUNK_WIDTH_IN_BLOCKS: u16 = 16;
pub const CHUNK_DEPTH_IN_BLOCKS: u16 = 16;

pub const WORLD_HEIGHT_IN_CHUNKS: u16 = 4;
pub const WORLD_WIDTH_IN_CHUNKS: u16 = 4;
pub const WORLD_DEPTH_IN_CHUNKS: u16 = 1;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Game),
            (spawn_world, spawn_light),
        ).add_systems(Update, mesh_chunk);
    }
}
