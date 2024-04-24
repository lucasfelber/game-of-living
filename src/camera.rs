use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 2., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}