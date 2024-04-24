use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use simulation::SimulationPlugin;

mod cell;
mod camera;
mod simulation;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin,
            SimulationPlugin
        ))
        .add_systems(Startup, (
            camera::spawn_camera,
            cell::spawn_cells,
        ))
        .add_systems(Update, cell::swap_cell_visibility)
        .run();
}
