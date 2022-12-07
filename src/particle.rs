use bevy::prelude::*;
use rand::Rng;

pub struct ParticlePlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ParticleSpawner {
    spawn_timer: Timer,
    spawn_amount: usize,
    particle_count: usize,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Particle {
    velocity: Vec3,
    color: Color,
    size: f32,
    lifetime: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_particle_spawner)
            .add_system(particle_spawner)
            .add_system(cleanup)
            .register_type::<Particle>();
    }
}

fn create_particle_spawner(mut commands: Commands) {
    commands.spawn({}).insert(ParticleSpawner {
        spawn_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
        spawn_amount: 100,
        particle_count: 0,
    });
}

fn particle_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world: Query<&mut ParticleSpawner>,
    time: Res<Time>,
) {
    let mut world = world.single_mut();
    world.spawn_timer.tick(time.delta());

    if world.spawn_timer.finished() {
        let mut rng = rand::thread_rng();
        println!("Particle count: {}", world.particle_count);

        // Spawn a `spawn_amount` of particles
        for _ in 0..world.spawn_amount {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.1,
                        subdivisions: 3,
                    })),
                    material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                    transform: Transform::from_xyz(
                        rng.gen_range(-5.0..5.0),
                        rng.gen_range(-5.0..5.0),
                        rng.gen_range(-5.0..5.0),
                    ),
                    ..Default::default()
                })
                .insert(Lifetime {
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                })
                .insert(Name::new(format!("Particle {}", world.particle_count)));
            world.particle_count += 1;
        }
    }
}

fn cleanup(mut commands: Commands, mut particles: Query<(Entity, &mut Lifetime)>, time: Res<Time>) {
    for (entity, mut lifetime) in &mut particles {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
