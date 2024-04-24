use bevy::{pbr::wireframe::WireframePlugin, prelude::*};

mod cell;
mod camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin,
        ))
        .add_systems(Startup, (
            camera::spawn_camera,
            cell::spawn_cells,
        ))
        .add_systems(Update, cell::swap_cell_visibility)
        .run();
}
