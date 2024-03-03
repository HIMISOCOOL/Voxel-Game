use bevy::{
    math::IVec3,
    prelude::{Component, Transform, Vec3},
    render::{
        mesh::{shape, Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
};

use super::{
    FACE_MASK_BACK, FACE_MASK_BOTTOM, FACE_MASK_FRONT, FACE_MASK_LEFT, FACE_MASK_RIGHT,
    FACE_MASK_TOP,
};

#[derive(Component)]
pub struct Chunk {
    pub updated: bool,
}

#[derive(Component)]
pub struct Voxel {
    pub solid: bool,
    pub mask: u8,
}

impl Default for Voxel {
    fn default() -> Self {
        Voxel {
            solid: false,
            mask: 0b000000,
        }
    }
}
impl Voxel {
    fn to_face_set(&self) -> (bool, bool, bool, bool, bool, bool) {
        let set = (
            self.mask & FACE_MASK_TOP == FACE_MASK_TOP,
            self.mask & FACE_MASK_BOTTOM == FACE_MASK_BOTTOM,
            self.mask & FACE_MASK_LEFT == FACE_MASK_LEFT,
            self.mask & FACE_MASK_RIGHT == FACE_MASK_RIGHT,
            self.mask & FACE_MASK_FRONT == FACE_MASK_FRONT,
            self.mask & FACE_MASK_BACK == FACE_MASK_BACK,
        );
        // println!("{0:#08b} = {1:?}", self.mask, set);
        set
    }
}

impl From<&Voxel> for Mesh {
    fn from(sp: &Voxel) -> Self {
        type Vertecies = ([f32; 3], [f32; 3], [f32; 2]);
        // TODO make voxel size a const
        let shape = shape::Box::new(0.1, 0.1, 0.1);
        let (top, bottom, left, right, front, back) = sp.to_face_set();
        // suppose Y-up right hand, and camera look from +z to -z
        let mut vertices: Vec<Vertecies> = vec![];
        let mut indices: Vec<u32> = vec![];
        if front {
            let current_indices_count = vertices.len() as u32;
            indices.extend_from_slice(&[
                current_indices_count,
                current_indices_count + 1,
                current_indices_count + 2,
                current_indices_count + 2,
                current_indices_count + 3,
                current_indices_count,
            ]);
            vertices.extend_from_slice(&[
                (
                    [shape.min_x, shape.min_y, shape.max_z],
                    [0., 0., 1.0],
                    [0., 0.],
                ),
                (
                    [shape.max_x, shape.min_y, shape.max_z],
                    [0., 0., 1.0],
                    [1.0, 0.],
                ),
                (
                    [shape.max_x, shape.max_y, shape.max_z],
                    [0., 0., 1.0],
                    [1.0, 1.0],
                ),
                (
                    [shape.min_x, shape.max_y, shape.max_z],
                    [0., 0., 1.0],
                    [0., 1.0],
                ),
            ]);
        }
        if back {
            let current_indices_count = vertices.len() as u32;
            indices.extend_from_slice(&[
                current_indices_count,
                current_indices_count + 1,
                current_indices_count + 2,
                current_indices_count + 2,
                current_indices_count + 3,
                current_indices_count,
            ]);
            vertices.extend_from_slice(&[
                (
                    [shape.min_x, shape.max_y, shape.min_z],
                    [0., 0., -1.0],
                    [1.0, 0.],
                ),
                (
                    [shape.max_x, shape.max_y, shape.min_z],
                    [0., 0., -1.0],
                    [0., 0.],
                ),
                (
                    [shape.max_x, shape.min_y, shape.min_z],
                    [0., 0., -1.0],
                    [0., 1.0],
                ),
                (
                    [shape.min_x, shape.min_y, shape.min_z],
                    [0., 0., -1.0],
                    [1.0, 1.0],
                ),
            ]);
        }
        if right {
            let current_indices_count = vertices.len() as u32;
            indices.extend_from_slice(&[
                current_indices_count,
                current_indices_count + 1,
                current_indices_count + 2,
                current_indices_count + 2,
                current_indices_count + 3,
                current_indices_count,
            ]);
            vertices.extend_from_slice(&[
                (
                    [shape.max_x, shape.min_y, shape.min_z],
                    [1.0, 0., 0.],
                    [0., 0.],
                ),
                (
                    [shape.max_x, shape.max_y, shape.min_z],
                    [1.0, 0., 0.],
                    [1.0, 0.],
                ),
                (
                    [shape.max_x, shape.max_y, shape.max_z],
                    [1.0, 0., 0.],
                    [1.0, 1.0],
                ),
                (
                    [shape.max_x, shape.min_y, shape.max_z],
                    [1.0, 0., 0.],
                    [0., 1.0],
                ),
            ]);
        }
        if left {
            let current_indices_count = vertices.len() as u32;
            indices.extend_from_slice(&[
                current_indices_count,
                current_indices_count + 1,
                current_indices_count + 2,
                current_indices_count + 2,
                current_indices_count + 3,
                current_indices_count,
            ]);
            vertices.extend_from_slice(&[
                (
                    [shape.min_x, shape.min_y, shape.max_z],
                    [-1.0, 0., 0.],
                    [1.0, 0.],
                ),
                (
                    [shape.min_x, shape.max_y, shape.max_z],
                    [-1.0, 0., 0.],
                    [0., 0.],
                ),
                (
                    [shape.min_x, shape.max_y, shape.min_z],
                    [-1.0, 0., 0.],
                    [0., 1.0],
                ),
                (
                    [shape.min_x, shape.min_y, shape.min_z],
                    [-1.0, 0., 0.],
                    [1.0, 1.0],
                ),
            ]);
        }
        if top {
            let current_indices_count = vertices.len() as u32;
            indices.extend_from_slice(&[
                current_indices_count,
                current_indices_count + 1,
                current_indices_count + 2,
                current_indices_count + 2,
                current_indices_count + 3,
                current_indices_count,
            ]);
            vertices.extend_from_slice(&[
                (
                    [shape.max_x, shape.max_y, shape.min_z],
                    [0., 1.0, 0.],
                    [1.0, 0.],
                ),
                (
                    [shape.min_x, shape.max_y, shape.min_z],
                    [0., 1.0, 0.],
                    [0., 0.],
                ),
                (
                    [shape.min_x, shape.max_y, shape.max_z],
                    [0., 1.0, 0.],
                    [0., 1.0],
                ),
                (
                    [shape.max_x, shape.max_y, shape.max_z],
                    [0., 1.0, 0.],
                    [1.0, 1.0],
                ),
            ]);
        }
        if bottom {
            let current_indices_count = vertices.len() as u32;
            indices.extend_from_slice(&[
                current_indices_count,
                current_indices_count + 1,
                current_indices_count + 2,
                current_indices_count + 2,
                current_indices_count + 3,
                current_indices_count,
            ]);
            vertices.extend_from_slice(&[
                (
                    [shape.max_x, shape.min_y, shape.max_z],
                    [0., -1.0, 0.],
                    [0., 0.],
                ),
                (
                    [shape.min_x, shape.min_y, shape.max_z],
                    [0., -1.0, 0.],
                    [1.0, 0.],
                ),
                (
                    [shape.min_x, shape.min_y, shape.min_z],
                    [0., -1.0, 0.],
                    [1.0, 1.0],
                ),
                (
                    [shape.max_x, shape.min_y, shape.min_z],
                    [0., -1.0, 0.],
                    [0., 1.0],
                ),
            ]);
        }

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_indices(Some(Indices::U32(indices)))
    }
}

#[derive(Component, Clone, Copy)]
pub struct ChunkCoordinate(pub u16, pub u16, pub u16);

impl ChunkCoordinate {
    pub fn into_ivec3(&self) -> IVec3 {
        IVec3 {
            x: self.0.clone().into(),
            y: self.1.clone().into(),
            z: self.2.clone().into(),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct WorldCoordinate(f32, f32, f32);

impl WorldCoordinate {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        WorldCoordinate(x, y, z)
    }

    pub fn from_vec3(vec3: Vec3) -> Self {
        WorldCoordinate(vec3.x, vec3.y, vec3.z)
    }

    pub fn into_translation(self) -> Vec3 {
        Vec3 {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }

    pub fn into_transform(self) -> Transform {
        Transform::from_translation(self.into_translation())
    }
}

impl Default for WorldCoordinate {
    fn default() -> Self {
        WorldCoordinate(0.0, 0.0, 0.0)
    }
}
