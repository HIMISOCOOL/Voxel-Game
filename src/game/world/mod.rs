use bevy::prelude::*;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

pub mod components;
pub mod resources;
pub mod systems;

pub const CHUNK_HEIGHT_IN_BLOCKS: u16 = 16;
pub const CHUNK_WIDTH_IN_BLOCKS: u16 = 16;
pub const CHUNK_DEPTH_IN_BLOCKS: u16 = 16;

pub const WORLD_HEIGHT_IN_CHUNKS: u16 = 4;
pub const WORLD_WIDTH_IN_CHUNKS: u16 = 4;
pub const WORLD_DEPTH_IN_CHUNKS: u16 = 1;

/**
 * No Face adjacencies
 */
pub const FACE_MASK_DEFAULT: u8 = 0b000000;
pub const FACE_MASK_TOP: u8 = 0b100000;
pub const FACE_MASK_BOTTOM: u8 = 0b010000;
pub const FACE_MASK_LEFT: u8 = 0b001000;
pub const FACE_MASK_RIGHT: u8 = 0b000100;
pub const FACE_MASK_FRONT: u8 = 0b000010;
pub const FACE_MASK_BACK: u8 = 0b000001;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), (spawn_world, spawn_light))
            .add_systems(Update, (update_chunk, mesh_chunk).chain());
    }
}
