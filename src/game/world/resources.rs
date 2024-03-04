use bevy::{math::IVec3, prelude::*, utils::HashMap};

pub struct Chunk {
    pub entity_id: Entity,
    pub blocks: HashMap<IVec3, Entity>,
}

#[derive(Resource)]
pub struct VoxelWorld {
    pub chunks: HashMap<IVec3, Chunk>,
    pub mesh_cache: HashMap<u8, Handle<Mesh>>,
}

#[derive(Resource)]
pub struct CubeMesh {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
}
