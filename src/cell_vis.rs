use bevy::{pbr::wireframe::{NoWireframe, Wireframe}, prelude::*};
use bevy_rapier3d::geometry::Collider;

use crate::GRID;

const CELL_SIZE: f32 = 1.;

#[derive(Component)]
pub struct Cell;

#[derive(Component)]
pub struct Position {pub x: usize, pub y: usize}

#[derive(Component)]
pub struct SwapMaterial(pub Handle<StandardMaterial>);

pub fn render_new_layer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board: [i8; GRID*GRID],
    layer: u32,
    cell_q: Query<Entity, (With<Cell>, Or<(With<Wireframe>, With<NoWireframe>)>)>,
) {
    for entity in cell_q.iter() {
        commands.entity(entity)
            .remove::<Collider>()
            .remove::<Wireframe>()
            .remove::<NoWireframe>();
    }

    for (index, state) in board.iter().enumerate() {
        if *state == 0 {
            commands.spawn((
                Cell,
                Collider::cuboid(CELL_SIZE/2., CELL_SIZE/4., CELL_SIZE/2.),
                Position {
                    x: index % GRID,
                    y: index / GRID,
                },
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(CELL_SIZE, CELL_SIZE/2., CELL_SIZE)),
                    material: materials.add(Color::rgba(0., 0., 0., 0.)),
                    transform: Transform::from_xyz(
                        (index % GRID) as f32 * CELL_SIZE,
                        (CELL_SIZE/2.) * layer as f32,
                        (index / GRID) as f32 * CELL_SIZE,
                    ),
                    ..default()
                },
                SwapMaterial(materials.add(Color::BLUE)),
                Wireframe,
                ));
        } else if *state == 1 {
            commands.spawn((
                Cell,
                Collider::cuboid(CELL_SIZE/2., CELL_SIZE/4., CELL_SIZE/2.),
                Position {
                    x: index % GRID,
                    y: index / GRID,
                },
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(CELL_SIZE, CELL_SIZE/2., CELL_SIZE)),
                    material: materials.add(Color::BLUE),
                    transform: Transform::from_xyz(
                        (index % GRID) as f32 * CELL_SIZE,
                        (CELL_SIZE/2.) * layer as f32,
                        (index / GRID) as f32 * CELL_SIZE,
                    ),
                    ..default()
                },
                SwapMaterial(materials.add(Color::rgba(0., 0., 0., 0.))),
                NoWireframe,
                ));
        }
    }
}