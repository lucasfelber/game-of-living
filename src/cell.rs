use bevy::{pbr::wireframe::{NoWireframe, Wireframe}, prelude::*};

use crate::GRID;

const CELL_SIZE: f32 = 1.;

#[derive(Component)]
pub struct Cell;

#[derive(Component)]
pub struct SwapMaterial(Handle<StandardMaterial>);

#[derive(Component)]
struct State(u8);

#[derive(Component)]
struct Position { x: u32, y: u32 }

pub fn spawn_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..GRID {
        for y in 0..GRID {
            commands.spawn((
                Cell,
                State(0),
                // Position{x, y},
                SwapMaterial(materials.add(Color::rgba(0., 0., 0., 0.))),
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(CELL_SIZE, CELL_SIZE/2., CELL_SIZE)),
                    material: materials.add(Color::BLUE),
                    transform: Transform::from_xyz(x as f32 * CELL_SIZE, 0., y as f32 * CELL_SIZE),
                    ..default()
                },
                NoWireframe,
            ));
        }
    }
}

pub fn swap_cell_visibility(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    wireframe: Query<(Entity, &Handle<StandardMaterial>, &SwapMaterial), (With<Wireframe>, With<Cell>)>,
    no_wireframe: Query<(Entity, &Handle<StandardMaterial>, &SwapMaterial), (With<NoWireframe>, With<Cell>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {

        for (cell, current_mat, swap_mat) in &no_wireframe {
            commands.entity(cell)
                .remove::<Handle<StandardMaterial>>()
                .remove::<SwapMaterial>()
                .remove::<NoWireframe>()
                .insert((
                    swap_mat.0.clone(),
                    SwapMaterial(current_mat.clone()),
                    Wireframe
                ));
        }

        for (cell, current_mat, swap_mat) in &wireframe {
            commands.entity(cell)
                .remove::<Handle<StandardMaterial>>()
                .remove::<SwapMaterial>()
                .remove::<Wireframe>()
                .insert((
                    swap_mat.0.clone(),
                    SwapMaterial(current_mat.clone()),
                    NoWireframe
                ));
        }

    }
}