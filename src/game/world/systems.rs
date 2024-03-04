use bevy::{prelude::*, utils::HashMap};
use rand::random;

use crate::game::SimulationState;

use super::{
    components,
    resources::{self, CubeMesh, VoxelWorld},
    CHUNK_DEPTH_IN_BLOCKS, CHUNK_HEIGHT_IN_BLOCKS, CHUNK_WIDTH_IN_BLOCKS, FACE_MASK_BACK,
    FACE_MASK_BOTTOM, FACE_MASK_DEFAULT, FACE_MASK_FRONT, FACE_MASK_LEFT, FACE_MASK_RIGHT,
    FACE_MASK_TOP, WORLD_DEPTH_IN_CHUNKS, WORLD_HEIGHT_IN_CHUNKS, WORLD_WIDTH_IN_CHUNKS,
};

pub fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 600.0,
                color: Color::WHITE.as_lcha(),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("World Light"),
    );

    commands.spawn(light);
}

type VoxelBundle = (
    components::ChunkCoordinate,
    components::Voxel,
    components::WorldCoordinate,
    Name,
);

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // takes an chunk position in chunk space and creates blocks inside it
    let spawn_chunk = |chunk_x: u16, chunk_y: u16, chunk_z: u16| -> Vec<VoxelBundle> {
        let mut blocks: Vec<VoxelBundle> = vec![];
        for x in 0..CHUNK_WIDTH_IN_BLOCKS {
            for y in 0..CHUNK_DEPTH_IN_BLOCKS {
                for z in 0..CHUNK_HEIGHT_IN_BLOCKS {
                    // figure out the transform for the block
                    let voxel_world_position = Vec3::new(
                        f32::from(x + (chunk_x * CHUNK_WIDTH_IN_BLOCKS)) * 0.1,
                        f32::from(y + (chunk_y * CHUNK_DEPTH_IN_BLOCKS)) * 0.1,
                        f32::from(z + (chunk_z * CHUNK_HEIGHT_IN_BLOCKS)) * 0.1,
                    );
                    // create a nice name for bevy inspector
                    let name = format!("Block ({x}, {y}, {z})");
                    // randomly make the block solid
                    // TODO use perlin nosie?
                    let is_solid = random::<bool>();

                    blocks.push((
                        components::ChunkCoordinate(x, y, z),
                        components::Voxel {
                            solid: is_solid,
                            ..default()
                        },
                        components::WorldCoordinate::from_vec3(voxel_world_position),
                        Name::new(name),
                    ));
                }
            }
        }
        blocks
    };

    let mut voxel_world = resources::VoxelWorld {
        chunks: HashMap::new(),
        mesh_cache: HashMap::new(),
    };

    // for each chunk in
    for x in 0..WORLD_WIDTH_IN_CHUNKS {
        for y in 0..WORLD_DEPTH_IN_CHUNKS {
            for z in 0..WORLD_HEIGHT_IN_CHUNKS {
                let name = format!("Chunk ({x}, {y}, {z})");

                let mut blocks: HashMap<IVec3, Entity> = HashMap::new();
                // spawn a chunk
                let chunk_id = commands
                    .spawn((
                        SpatialBundle::default(),
                        components::Chunk { updated: true },
                        components::WorldCoordinate::from_xyz(x as f32, y as f32, z as f32),
                        Name::new(name),
                    ))
                    // and add children
                    .with_children(|parent| {
                        for (chunk_coordinate, voxel, world_coordinate, name) in
                            spawn_chunk(x, y, z)
                        {
                            let block_id = parent
                                .spawn((chunk_coordinate, voxel, world_coordinate, name))
                                .id();
                            blocks.insert(chunk_coordinate.into_ivec3(), block_id);
                        }
                    })
                    .id();
                voxel_world.chunks.insert(
                    IVec3 {
                        x: x.into(),
                        y: y.into(),
                        z: z.into(),
                    },
                    resources::Chunk {
                        entity_id: chunk_id,
                        blocks,
                    },
                );
            }
        }
    }
    commands.insert_resource(resources::CubeMesh {
        mesh_handle: meshes.add(Mesh::from(shape::Cube::new(0.1))),
        material_handle: materials.add(Color::SEA_GREEN.into()),
    });
    commands.insert_resource(voxel_world);
}

pub fn update_chunk(
    voxel_world: Res<resources::VoxelWorld>,
    chunk_query: Query<(&components::Chunk, &components::WorldCoordinate, &Children)>,
    mut block_query: Query<(&components::ChunkCoordinate, &mut components::Voxel)>,
    simulation_state: Res<State<SimulationState>>,
) {
    if *simulation_state.get() == SimulationState::Paused {
        return;
    }
    for (chunk, chunk_world_coordinate, children) in chunk_query.iter() {
        if !chunk.updated {
            continue;
        }
        // for each block
        for block_entity in children.iter() {
            if let Some(chunk_resource) = voxel_world
                .chunks
                .get(&chunk_world_coordinate.into_translation().as_ivec3())
            {
                if let Ok((block_chunk_coordinate, voxel)) = block_query.get(*block_entity) {
                    if !voxel.solid {
                        continue;
                    }

                    // find the adjacent blocks
                    // build flags for each face
                    let mut mask = FACE_MASK_DEFAULT;

                    let IVec3 { x, y, z } = block_chunk_coordinate.into_ivec3();

                    // if there is a block to the right
                    if let Some(next_block_entity) =
                        chunk_resource.blocks.get(&IVec3 { x: x + 1, y, z })
                    {
                        if let Ok((_, adjacent_voxel)) = block_query.get(*next_block_entity) {
                            // but its not solid
                            if !adjacent_voxel.solid {
                                mask |= FACE_MASK_RIGHT;
                            }
                        }
                    // otherwise theres no block
                    } else {
                        mask |= FACE_MASK_RIGHT;
                    }
                    // if there is a block to the left
                    if let Some(prev_block_entity) =
                        chunk_resource.blocks.get(&IVec3 { x: x - 1, y, z })
                    {
                        if let Ok((_, adjacent_voxel)) = block_query.get(*prev_block_entity) {
                            // but its not solid
                            if !adjacent_voxel.solid {
                                mask |= FACE_MASK_LEFT;
                            }
                        }
                    // otherwise theres no block
                    } else {
                        mask |= FACE_MASK_LEFT;
                    }

                    if let Some(prev_block_entity) =
                        chunk_resource.blocks.get(&IVec3 { x, y: y + 1, z })
                    {
                        if let Ok((_, adjacent_voxel)) = block_query.get(*prev_block_entity) {
                            if !adjacent_voxel.solid {
                                mask |= FACE_MASK_TOP;
                            }
                        }
                    } else {
                        mask |= FACE_MASK_TOP;
                    }
                    if let Some(next_block_entity) =
                        chunk_resource.blocks.get(&IVec3 { x, y: y - 1, z })
                    {
                        if let Ok((_, adjacent_voxel)) = block_query.get(*next_block_entity) {
                            if !adjacent_voxel.solid {
                                mask |= FACE_MASK_BOTTOM;
                            }
                        }
                    } else {
                        mask |= FACE_MASK_BOTTOM;
                    }

                    if let Some(prev_block_entity) =
                        chunk_resource.blocks.get(&IVec3 { x, y, z: z + 1 })
                    {
                        if let Ok((_, adjacent_voxel)) = block_query.get(*prev_block_entity) {
                            if !adjacent_voxel.solid {
                                mask |= FACE_MASK_FRONT;
                            }
                        }
                    } else {
                        mask |= FACE_MASK_FRONT;
                    }
                    if let Some(next_block_entity) =
                        chunk_resource.blocks.get(&IVec3 { x, y, z: z - 1 })
                    {
                        if let Ok((_, adjacent_voxel)) = block_query.get(*next_block_entity) {
                            if !adjacent_voxel.solid {
                                mask |= FACE_MASK_BACK;
                            }
                        }
                    } else {
                        mask |= FACE_MASK_BACK;
                    }

                    if let Ok((_, mut voxel)) = block_query.get_mut(*block_entity) {
                        voxel.mask = mask;
                    }
                }
            }
        }
    }
}

pub fn mesh_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    cube_mesh: Res<CubeMesh>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut voxel_world: ResMut<VoxelWorld>,
    mut chunk_query: Query<(&mut components::Chunk, &Children)>,
    voxel_query: Query<(&components::WorldCoordinate, &components::Voxel)>,
    simulation_state: Res<State<SimulationState>>,
) {
    if *simulation_state.get() == SimulationState::Paused {
        return;
    }
    // TODO reuse material
    // let material_handle = materials.add(Color::SEA_GREEN.into());
    // for each chunk
    for (mut chunk, children) in chunk_query.iter_mut() {
        // if the chunk was not updated
        if !chunk.updated {
            // skip this loop
            continue;
        }
        // for each voxel in the chunk
        for entity in children.iter() {
            // get the voxel from the ecs
            if let Ok((world_coordinate, voxel)) = voxel_query.get(*entity) {
                if voxel.solid || voxel.mask != FACE_MASK_DEFAULT {
                    // add an updated mesh
                    // TODO remove the old mesh?
                    if !voxel_world.mesh_cache.contains_key(&voxel.mask) {
                        voxel_world
                            .mesh_cache
                            .insert(voxel.mask, meshes.add(Mesh::from(voxel)));
                    }
                    let mesh = voxel_world.mesh_cache.get(&voxel.mask).unwrap().clone();
                    let material = cube_mesh.material_handle.clone();
                    commands.entity(*entity).insert(PbrBundle {
                        mesh,
                        transform: world_coordinate.into_transform(),
                        material,
                        ..default()
                    });
                }
            }
        }
        // chunk has now been updated
        chunk.updated = false;
    }
}
