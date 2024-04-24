use bevy::prelude::*;

use crate::cell_calc::{calculate_board, CellState};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .init_resource::<SimulationTimer>()
            .add_systems(Update, (
                change_simulation_state,
                change_simulation_speed,
                simulation_step,
            ));
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum SimulationState {
    #[default]
    Editing,
    Running,
}

fn change_simulation_state(
    state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        match state.get() {
            SimulationState::Editing => next_state.set(SimulationState::Running),
            SimulationState::Running => next_state.set(SimulationState::Editing)
        }
    }
}

#[derive(Resource)]
struct SimulationTimer(Timer);

impl Default for SimulationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5., TimerMode::Repeating))
    }
}

fn change_simulation_speed(
    mut timer: ResMut<SimulationTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyK) {
        *timer = SimulationTimer(Timer::new(timer.0.duration() / 2, TimerMode::Repeating))
    }
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        *timer = SimulationTimer(Timer::new(timer.0.duration() * 2, TimerMode::Repeating))
    }
}

fn simulation_step(
    time: Res<Time>,
    mut timer: ResMut<SimulationTimer>,
    state: Res<State<SimulationState>>,
    board: Res<State<CellState>>,
    next_board: ResMut<NextState<CellState>>,
) {
    match state.get() {
        SimulationState::Running => {
            if timer.0.tick(time.delta()).finished() {
                calculate_board(board, next_board)
            }
        },
        SimulationState::Editing => {}
    }
}