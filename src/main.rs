use std::f32::consts::PI;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
mod particle;
mod world;

use bevy_egui::{egui, EguiContext, EguiPlugin};
use particle::{Lifetime, ParticlePlugin};
use world::WorldPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 1 })
        .init_resource::<GlobalState>()
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_scene)
        .add_startup_system(configure_global_state)
        .add_system(camera_controls)
        .add_system(bevy::window::close_on_esc)
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
        .add_plugin(WorldPlugin)
        .add_plugin(ParticlePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(fps_update_system)
        .add_system(particle_count_update_system)
        .add_system(ui)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(25.0, 0.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // DIRECTIONAL LIGHT
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    // AMBIENT LIGHT
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.15,
    });

    // FPS TEXT
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..Default::default()
        }),
        FpsText,
    ));

    // PARTICLE COUNT TEXT
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "PARTICLES: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..Default::default()
        }),
        NumOfParticlesText,
    ));
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

    let speed = 10.0;
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

#[derive(Default, Resource)]
struct GlobalState {
    base_velocity: f32,
    spawn_time: f32,
    spawn_amount: u32,
    lifetime: f32,
    bounds: bool,
    gravity_strength: f32,
    example: bool,
}

fn configure_global_state(mut state: ResMut<GlobalState>) {
    state.base_velocity = 2.0;
    state.spawn_time = 0.5;
    state.spawn_amount = 10;
    state.lifetime = 5.0;
    state.bounds = true;
    state.example = true;
    state.gravity_strength = 9.81;
}

fn ui(mut egui_context: ResMut<EguiContext>, mut state: ResMut<GlobalState>) {
    egui::Window::new("Menu")
        .default_size([300.0, 100.0])
        .show(egui_context.ctx_mut(), |ui| {
            // Slider
            ui.add(egui::Slider::new(&mut state.base_velocity, 1.0..=20.0).text("Base velocity"));
            ui.add(egui::Slider::new(&mut state.lifetime, 0.5..=10.0).text("Lifetime"));
            ui.add(egui::Slider::new(&mut state.spawn_amount, 1..=50).text("Spawn amount"));
            ui.add(egui::Slider::new(&mut state.spawn_time, 0.1..=2.0).text("Spawn time"));
            ui.add(
                egui::Slider::new(&mut state.gravity_strength, 0.0..=20.0).text("Gravity strength"),
            );

            // Checkbox
            ui.add(egui::Checkbox::new(&mut state.bounds, "Bounds"));
            ui.add(egui::Checkbox::new(&mut state.example, "Example"));
        });
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct NumOfParticlesText;

fn fps_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn particle_count_update_system(
    mut query: Query<&mut Text, With<NumOfParticlesText>>,
    query2: Query<&Lifetime>,
) {
    for mut text in &mut query {
        let count = query2.iter().len();
        text.sections[1].value = format!("{}", count);
    }
}
