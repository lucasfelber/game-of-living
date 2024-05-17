use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::{Real, *};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickEvent>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_ray);
    }
}

fn spawn_camera(mut commands: Commands) {
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

#[derive(Event)]
pub struct ClickEvent(pub Entity);

fn camera_ray(
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut click_evw: EventWriter<ClickEvent>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let (camera, transform) = camera.single();

        if let Some(camera_ray) = window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(transform, cursor))
        {
            if let Some((entity, _)) = rapier_context.cast_ray(
                camera_ray.origin,
                camera_ray.direction.into(),
                Real::MAX,
                true,
                QueryFilter::new(),
            ) {
                click_evw.send(ClickEvent(entity));
            }
        }
    }
}