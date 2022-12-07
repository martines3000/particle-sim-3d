use bevy::{prelude::*, window::PresentMode};
mod particle;
mod world;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy_inspector_egui::WorldInspectorPlugin;
use particle::ParticlePlugin;
use world::WorldPlugin;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_scene)
        .add_system(camera_controls)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "3D Particle simulation".to_string(),
                width: 500.0,
                height: 500.0,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(WorldPlugin)
        .add_plugin(ParticlePlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 2.0;
    let rotation_speed = 0.3;

    if keyboard_input.pressed(KeyCode::W) {
        camera.translation += forward * speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        camera.translation -= forward * speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::A) {
        camera.translation += left * speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::D) {
        camera.translation -= left * speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotation_speed * time.delta_seconds());
    }

    if keyboard_input.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotation_speed * time.delta_seconds());
    }

    if keyboard_input.pressed(KeyCode::C) {
        camera.rotate_axis(Vec3::X, rotation_speed * time.delta_seconds());
    }

    if keyboard_input.pressed(KeyCode::X) {
        camera.rotate_axis(Vec3::X, -rotation_speed * time.delta_seconds());
    }
}
