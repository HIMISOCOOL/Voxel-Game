use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;
use systems::*;

mod create_world;
pub mod events;
mod game;
mod main_menu;
mod options;
mod systems;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin, WorldInspectorPlugin::new()))
        .add_state::<AppState>()
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
    Death,
    Options,
    CreateWorld,
    LoadWorld,
    Multiplayer,
}
