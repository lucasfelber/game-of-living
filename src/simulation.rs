use bevy::{pbr::wireframe::{NoWireframe, Wireframe}, prelude::*};

use crate::{camera::ClickEvent, cell_calc::{self, Board}, cell_vis::{self, Cell, Position, SwapMaterial}, GRID};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .init_resource::<SimulationTime>()
            .add_systems(Startup, initial_step)
            .add_systems(Update, (
                change_simulation_state,
                change_simulation_speed,
                simulation_step,
                swap_cell_click,
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
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            SimulationState::Editing => next_state.set(SimulationState::Running),
            SimulationState::Running => next_state.set(SimulationState::Editing)
        }
    }
}

#[derive(Resource)]
struct SimulationTime {
    timer: Timer,
    steps: u32,
}

impl Default for SimulationTime {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2., TimerMode::Repeating),
            steps: 0,
        }
    }
}

fn change_simulation_speed(
    mut sim_time: ResMut<SimulationTime>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyK) {
        sim_time.timer = Timer::new(sim_time.timer.duration() / 2, TimerMode::Repeating)
    }
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        sim_time.timer = Timer::new(sim_time.timer.duration() * 2, TimerMode::Repeating)
    }
}

fn initial_step(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    board: Res<State<Board>>,
    cell_q: Query<Entity, (With<Cell>, Or<(With<Wireframe>, With<NoWireframe>)>)>,
) {
    cell_vis::render_new_layer(commands, meshes, materials, board.get().0, 0, cell_q);
}

fn simulation_step(
    time: Res<Time>,
    mut sim_time: ResMut<SimulationTime>,
    state: Res<State<SimulationState>>,
    board: Res<State<Board>>,
    mut next_board: ResMut<NextState<Board>>,

    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    cell_q: Query<Entity, (With<Cell>, Or<(With<Wireframe>, With<NoWireframe>)>)>,
) {
    match state.get() {
        SimulationState::Running => {
            if sim_time.timer.tick(time.delta()).finished() {
                sim_time.steps = sim_time.steps + 1;
                let new_board = cell_calc::calculate_board(board.get().0);
                cell_vis::render_new_layer(commands, meshes, materials, new_board, sim_time.steps, cell_q);
                next_board.set(Board(new_board));
            }
        },
        SimulationState::Editing => {}
    }
}

fn swap_cell_click(
    mut commands: Commands,
    mut click_evr: EventReader<ClickEvent>,
    cell_q: Query<(&Position, &Handle<StandardMaterial>, &SwapMaterial), With<Cell>>,
    board: Res<State<Board>>,
    mut next_board: ResMut<NextState<Board>>,
) {
    for ev in click_evr.read() {
        let (position, current_mat, swap_mat) = cell_q.get(ev.0).unwrap();

        let mut new_board = board.get().0;

        match new_board[position.y * GRID + position.x] {
            0 => {
                new_board[position.y * GRID + position.x] = 1;
                commands.entity(ev.0)
                    .remove::<Handle<StandardMaterial>>()
                    .remove::<SwapMaterial>()
                    .remove::<Wireframe>()
                    .insert((
                        swap_mat.0.clone(),
                        SwapMaterial(current_mat.clone()),
                        NoWireframe,
                    ));
            },
            1 => {
                new_board[position.y * GRID + position.x] = 0;
                commands.entity(ev.0)
                    .remove::<Handle<StandardMaterial>>()
                    .remove::<SwapMaterial>()
                    .remove::<NoWireframe>()
                    .insert((
                        swap_mat.0.clone(),
                        SwapMaterial(current_mat.clone()),
                        Wireframe,
                    ));
            },
            _ => {},
        }
        
        next_board.set(Board(new_board));
    }
}