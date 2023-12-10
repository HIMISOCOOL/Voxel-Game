use bevy::prelude::*;
use rand::random;

use super::{
    components::{Block, Chunk, Coordinate},
    CHUNK_DEPTH_IN_BLOCKS, CHUNK_HEIGHT_IN_BLOCKS, CHUNK_WIDTH_IN_BLOCKS, WORLD_DEPTH_IN_CHUNKS,
    WORLD_HEIGHT_IN_CHUNKS, WORLD_WIDTH_IN_CHUNKS,
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

// pub fn spawn_floor(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let floor = (
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Plane::from_size(25.0))),
//             material: materials.add(Color::DARK_GREEN.into()),
//             ..default()
//         },
//         Name::new("Floor"),
//     );

//     commands.spawn(floor);
// }

pub fn spawn_world(mut commands: Commands) {
    // let mesh_handle = meshes.add(Mesh::from(shape::Cube::new(0.1)));
    // let material_handle = materials.add(Color::SEA_GREEN.into());

    // takes an chunk position in chunk space and creates block inside it
    let spawn_chunk =
        |chunk_x: u16, chunk_y: u16, chunk_z: u16| -> Vec<(Block, Coordinate, Name)> {
            let mut blocks: Vec<(Block, Coordinate, Name)> = vec![];
            for x in 0..CHUNK_WIDTH_IN_BLOCKS {
                for y in 0..CHUNK_DEPTH_IN_BLOCKS {
                    for z in 0..CHUNK_HEIGHT_IN_BLOCKS {
                        // figure out the transform for the block
                        let block_position = Vec3::new(
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
                            Block { solid: is_solid },
                            Coordinate::from_vec3(block_position),
                            Name::new(name),
                        ));
                    }
                }
            }
            blocks
        };

    // for each chunk in
    for x in 0..WORLD_WIDTH_IN_CHUNKS {
        for y in 0..WORLD_DEPTH_IN_CHUNKS {
            for z in 0..WORLD_HEIGHT_IN_CHUNKS {
                let name = format!("Chunk ({x}, {y}, {z})");
                // spawn a chunk
                commands
                    .spawn((
                        SpatialBundle::default(),
                        Chunk { updated: true },
                        Coordinate::from_xyz(x as f32, y as f32, z as f32),
                        Name::new(name),
                    ))
                    // and add children
                    .with_children(|parent| {
                        for block in spawn_chunk(x, y, z) {
                            parent.spawn(block);
                        }
                    });
            }
        }
    }
}

pub fn mesh_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_query: Query<(&mut Chunk, &Children)>,
    block_query: Query<(&Coordinate, &Block), With<Block>>,
) {
    // TODO make real mesh
    let mesh_handle = meshes.add(Mesh::from(shape::Cube::new(0.1)));
    // TODO reuse material
    let material_handle = materials.add(Color::SEA_GREEN.into());
    // for each chunk
    for (mut chunk, children) in chunk_query.iter_mut() {
        if !chunk.updated {
            return;
        }
        for block_entity in children.iter() {
            if let Ok((coordinate, block)) = block_query.get(*block_entity) {
                if block.solid {
                    // add an updated mesh
                    // TODO remove the old mesh
                    commands.entity(*block_entity).insert(PbrBundle {
                        mesh: mesh_handle.clone(),
                        transform: coordinate.into_transform(),
                        material: material_handle.clone(),
                        ..default()
                    });
                }
            }
        }
        chunk.updated = false;
    }
}
