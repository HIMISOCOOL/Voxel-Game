use bevy::prelude::{Component, Transform, Vec3};

#[derive(Component)]
pub struct Chunk {
    pub updated: bool,
}

#[derive(Component)]
pub struct Block {
    pub solid: bool,
}

impl Default for Block {
    fn default() -> Self {
        Block { solid: false }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Coordinate(f32, f32, f32);

impl Coordinate {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Coordinate(x, y, z)
    }

    pub fn from_vec3(vec3: Vec3) -> Self {
        Coordinate(vec3.x, vec3.y, vec3.z)
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

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate(0.0, 0.0, 0.0)
    }
}
