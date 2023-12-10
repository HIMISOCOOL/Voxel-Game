use bevy::prelude::*;

use crate::AppState;

use self::{camera::CameraPlugin, systems::*, world::WorldPlugin};

pub struct GamePlugin;

mod camera;
mod systems;
mod world;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // plugins
            .add_plugins((WorldPlugin, CameraPlugin))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
