mod systems;

use self::systems::*;
use bevy::prelude::*;
use bevy_flycam::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NoCameraPlayerPlugin)
            .add_systems(Startup, spawn_camera);
    }
}
