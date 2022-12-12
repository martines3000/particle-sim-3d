use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

use crate::GlobalState;

const DAMPING: f32 = 0.8;
const WORLD_SIZE: f32 = 10.0;
const ALPHA: f32 = 2.0;
const BETA: f32 = 0.1;

pub struct ParticlePlugin;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ParticleSpawner {
    spawn_timer: Timer,
    particle_count: usize,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ParticleVelocity {
    velocity: Vec3,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ParticleSize {
    size: f32,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_particle_spawner)
            .add_system(particle_spawner)
            .add_system(cleanup)
            .add_system(update_particle_location)
            .register_type::<ParticleVelocity>();
    }
}

fn create_particle_spawner(mut commands: Commands) {
    commands.spawn({}).insert(ParticleSpawner {
        spawn_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        particle_count: 0,
    });
}

fn particle_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world: Query<&mut ParticleSpawner>,
    time: Res<Time>,
    state: Res<GlobalState>,
) {
    let mut world = world.single_mut();
    world.spawn_timer.tick(time.delta());

    if world.spawn_timer.finished() {
        let mut rng = rand::thread_rng();
        let mut size;

        // Spawn a `spawn_amount` of particles
        for _ in 0..state.spawn_amount {
            size = rng.gen_range(0.1..1.0);
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: size,
                        subdivisions: 3,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            0.8,
                        )
                        .into(),
                        // emissive: Color::rgb(1.0, 0.0, 0.0).into(),
                        alpha_mode: AlphaMode::Blend,
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(
                        rng.gen_range(-4.0..4.0),
                        0.0,
                        rng.gen_range(-4.0..4.0),
                    ),
                    ..Default::default()
                })
                .insert(Lifetime {
                    timer: Timer::from_seconds(state.lifetime, TimerMode::Once),
                })
                .insert(ParticleVelocity {
                    velocity: Vec3::new(
                        rng.gen_range(-1.0..1.0) * state.base_velocity,
                        1.0 * state.base_velocity,
                        rng.gen_range(-1.0..1.0) * state.base_velocity,
                    ),
                })
                .insert(ParticleSize { size: size })
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

fn update_particle_location(
    mut particles: Query<(Entity, &ParticleSize, &mut ParticleVelocity, &mut Transform)>,
    time: Res<Time>,
    state: Res<GlobalState>,
) {
    let mut map = HashMap::new();
    if state.example {
        for (entity, size, velocity, transform) in &particles {
            for (entity_2, size_2, velocity_2, transform_2) in &particles {
                if (transform.translation - transform_2.translation)
                    .dot(velocity.velocity - velocity_2.velocity)
                    < 0.0
                    && transform.translation.distance(transform_2.translation)
                        < (size.size + size_2.size)
                {
                    let n1 = (transform_2.translation - transform.translation)
                        / transform_2.translation.distance(transform.translation);
                    let n2 = (transform.translation - transform_2.translation)
                        / transform.translation.distance(transform_2.translation);

                    let v1 = velocity.velocity - (1.0 + DAMPING) * n2.dot(velocity.velocity) * n2;
                    let v2 =
                        velocity_2.velocity - (1.0 + DAMPING) * n1.dot(velocity_2.velocity) * n1;

                    if !map.contains_key(&entity.index()) {
                        map.insert(entity.index(), v1);
                    }

                    if !map.contains_key(&entity_2.index()) {
                        map.insert(entity_2.index(), v2);
                    }
                }
            }

            // Collision with box
            if state.bounds {
                let v;
                if !map.contains_key(&entity.index()) {
                    map.insert(entity.index(), velocity.velocity);
                }
                v = map.get_mut(&entity.index()).unwrap();

                if transform.translation.x < -WORLD_SIZE {
                    *v -= (1.0 + DAMPING)
                        * (v.dot(Vec3::new(1.0, 0.0, 0.0)))
                        * Vec3::new(1.0, 0.0, 0.0);
                }

                if transform.translation.x > WORLD_SIZE {
                    *v -= (1.0 + DAMPING)
                        * (v.dot(Vec3::new(-1.0, 0.0, 0.0)))
                        * Vec3::new(-1.0, 0.0, 0.0);
                }

                if transform.translation.z < -WORLD_SIZE {
                    *v -= (1.0 + DAMPING)
                        * (v.dot(Vec3::new(0.0, 0.0, 1.0)))
                        * Vec3::new(0.0, 0.0, 1.0);
                }

                if transform.translation.z > WORLD_SIZE {
                    *v -= (1.0 + DAMPING)
                        * (v.dot(Vec3::new(0.0, 0.0, -1.0)))
                        * Vec3::new(0.0, 0.0, -1.0);
                }

                if transform.translation.y < -WORLD_SIZE {
                    *v -= (1.0 + DAMPING)
                        * (v.dot(Vec3::new(0.0, 1.0, 0.0)))
                        * Vec3::new(0.0, 1.0, 0.0);
                }

                if transform.translation.y > WORLD_SIZE {
                    *v -= (1.0 + DAMPING)
                        * (v.dot(Vec3::new(0.0, -1.0, 0.0)))
                        * Vec3::new(0.0, -1.0, 0.0);
                }
            }
        }

        for (entity, _, mut velocity, mut transform) in &mut particles {
            if map.contains_key(&entity.index()) {
                velocity.velocity = *map.get(&entity.index()).unwrap();
            }
            // Apply gravity
            velocity.velocity.y -= state.gravity_strength * time.delta_seconds();
            transform.translation += velocity.velocity * time.delta_seconds();
        }
    } else {
        // Direction
        let mut d;
        for (entity, _, velocity, transform) in &particles {
            d = Vec3::new(0.0, 0.0, 0.0);
            for (_, _, _, transform_2) in &particles {
                d += (transform.translation - transform_2.translation)
                    / (transform_2.translation.distance(transform.translation) + 1.0).powf(ALPHA);
            }

            if state.bounds {
                // Distance to planes of sorounding box
                if (transform.translation.x - WORLD_SIZE).abs() < 2.0 {
                    d.x = -(transform.translation.x - WORLD_SIZE).abs()
                        / ((transform.translation.x - WORLD_SIZE).abs() + 1.0).powf(ALPHA);
                }

                if (transform.translation.x + WORLD_SIZE).abs() < 2.0 {
                    d.x = (transform.translation.x + WORLD_SIZE).abs()
                        / ((transform.translation.x + WORLD_SIZE).abs() + 1.0).powf(ALPHA);
                }

                if (transform.translation.z - WORLD_SIZE).abs() < 2.0 {
                    d.z = -(transform.translation.z - WORLD_SIZE).abs()
                        / ((transform.translation.z - WORLD_SIZE).abs() + 1.0).powf(ALPHA);
                }

                if (transform.translation.z + WORLD_SIZE).abs() < 2.0 {
                    d.z = (transform.translation.z + WORLD_SIZE).abs()
                        / ((transform.translation.z + WORLD_SIZE).abs() + 1.0).powf(ALPHA);
                }

                if (transform.translation.y - WORLD_SIZE).abs() < 2.0 {
                    d.y = -(transform.translation.y - WORLD_SIZE).abs()
                        / ((transform.translation.y - WORLD_SIZE).abs() + 1.0).powf(ALPHA);
                }

                if (transform.translation.y + WORLD_SIZE).abs() < 2.0 {
                    d.y = (transform.translation.y + WORLD_SIZE).abs()
                        / ((transform.translation.y + WORLD_SIZE).abs() + 1.0).powf(ALPHA);
                }
            }

            map.insert(entity.index(), (1.0 - BETA) * velocity.velocity + BETA * d);
        }

        for (entity, _, mut velocity, mut transform) in &mut particles {
            if map.contains_key(&entity.index()) {
                velocity.velocity = *map.get(&entity.index()).unwrap();
            }

            // Apply gravity
            velocity.velocity.y -= state.gravity_strength * time.delta_seconds();
            transform.translation += velocity.velocity * time.delta_seconds();
        }
    }
}
