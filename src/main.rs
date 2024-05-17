use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use camera::CameraPlugin;
use cell_calc::CellCalcPlugin;
use simulation::SimulationPlugin;

mod cell_vis;
mod cell_calc;
mod camera;
mod simulation;

const GRID: usize = 5;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            SimulationPlugin,
            CellCalcPlugin,
            CameraPlugin,
        ))
        .run();
}
