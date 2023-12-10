use bevy::prelude::*;

// use crate::AppState;
use super::SimulationState;

// pub fn transition_to_pause_state(
//     keyboard_input: Res<Input<KeyCode>>,
//     app_state: Res<State<AppState>>,
//     next_simulation_state: ResMut<NextState<SimulationState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Escape) {
//         if *app_state.get() == AppState::Game {
//             pause_simulation(next_simulation_state);
//         }
//     }
// }

// pub fn transition_to_game_state(
//     keyboard_input: Res<Input<KeyCode>>,
//     app_state: Res<State<AppState>>,
//     next_simulation_state: ResMut<NextState<SimulationState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Escape) {
//         if *app_state.get() == AppState::Game {
//             resume_simulation(next_simulation_state);
//         }
//     }
// }

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let current_state = simulation_state.get();
        if *current_state == SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }
        if *current_state == SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Resumed.");
        }
    }
}
