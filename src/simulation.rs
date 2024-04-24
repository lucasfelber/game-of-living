use bevy::prelude::*;

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
        Self(Timer::from_seconds(2., TimerMode::Repeating))
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
) {
    match state.get() {
        SimulationState::Running => {
            if timer.0.tick(time.delta()).finished() {
                println!("STEP! at {:?}", timer.0.duration())
            }
        },
        SimulationState::Editing => println!("paused")
    }
}