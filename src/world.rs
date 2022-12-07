use bevy::prelude::*;

pub struct WorldPlugin;

const WORLD_HEIGHT: f32 = 5.0;
const WORLD_WIDTH: f32 = 5.0;
const WORLD_DEPTH: f32 = 5.0;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct World {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub gravity: Vec3,
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_world)
            .register_type::<World>();
    }
}

fn create_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn transparent cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                WORLD_WIDTH,
                WORLD_HEIGHT,
                WORLD_DEPTH,
            ))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.5, 0.5, 1.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(World {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            depth: WORLD_DEPTH,
            gravity: Vec3::new(0.0, -9.81, 0.0),
        });
}
