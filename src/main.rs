use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use cell_calc::CellCalcPlugin;
use simulation::SimulationPlugin;

mod cell;
mod cell_calc;
mod camera;
mod simulation;

const GRID: usize = 4;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin,
            SimulationPlugin,
            CellCalcPlugin
        ))
        .add_systems(Startup, (
            camera::spawn_camera,
            cell::spawn_cells,
        ))
        .add_systems(Update, cell::swap_cell_visibility)
        .run();
}
