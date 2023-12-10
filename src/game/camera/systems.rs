use bevy::prelude::*;
use bevy_flycam::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-1.0, 2.4, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam,
        Name::new("Camera"),
    );

    commands.spawn(camera);
}
