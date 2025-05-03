use bevy::{color::palettes::css::*, prelude::*};

use std::time::Duration;

#[derive(Component)]
struct Quakka;

#[derive(Component)]
struct Attacker {
    cooldown: Timer,
    damage: f32,
}

#[derive(Component)]
#[require(Chaseable)]
struct Farmer;

#[derive(Component)]
struct Health {
    current_health: f32,
    max_health: f32,
}

#[derive(Component)]
#[require(Transform)]
struct HealthBar;

#[derive(Component, Default)]
struct Chaseable;

#[derive(Resource, PartialEq)]
struct Paused(bool);

impl Default for Paused {
    fn default() -> Self {
        Paused(false)
    }
}

const QUAKKA_SPEED: f32 = 40.0;
const QUAKKA_HIT_DISTANCE: f32 = 50.0;
const QUAKKA_DAMAGE: f32 = 60.0;

const FARMER_SPEED: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            //level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_systems(Startup, (setup_camera, spawn_entities))
        .add_systems(
            FixedUpdate,
            (
                (
                    quakka_chase_and_attack,
                    delete_dead_entities,
                    update_healthbars,
                )
                    .chain()
                    .run_if(resource_equals(Paused(false))),
                (
                    move_farmer_with_wasd
                ).run_if(resource_equals(Paused(false))),
                tick_attacker_cooldowns,
            ),
        )
        .init_resource::<Paused>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn tick_attacker_cooldowns(mut attackers: Query<&mut Attacker>, time: Res<Time>) {
    for mut attacker in attackers.iter_mut() {
        if attacker.cooldown.mode() == TimerMode::Repeating {
            panic!("Attack cooldown should be once");
        }
        attacker.cooldown.tick(time.delta());
    }
}

fn move_farmer_with_wasd(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    farmer_transform_q: Single<&mut Transform, With<Farmer>>
) {
    let mut movement: Vec3 = Vec3::splat(0.0);
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        movement.x += 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement.y -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        movement.x -= 1.;
    }

    let mut transform = farmer_transform_q.into_inner();
    transform.translation += movement;
}

fn delete_dead_entities(
    healths: Query<(&Health, Entity)>,
    mut commands: Commands
) {
    for (health, e) in healths.iter() {
        if health.current_health <= 0.0 {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn quakka_chase_and_attack(
    mut quakkas: Query<(&mut Transform, &mut Attacker), (With<Quakka>, )>,
    mut chaseables: Query<(&Transform, Entity, &mut Health), (With<Chaseable>, Without<Quakka>)>,
    time: Res<Time>,
) {
    for mut quakka in quakkas.iter_mut() {
        let closest_chaseable = chaseables
            .iter_mut()
            .max_by(|a, b| {
                let a_distance = quakka.0.translation.distance(a.0.translation);
                let b_distance = quakka.0.translation.distance(b.0.translation);
                b_distance.partial_cmp(&a_distance).unwrap()
            });

        // There are no chaseables
        if closest_chaseable.is_none() {
            continue;
        }

        let mut closest_chaseable = closest_chaseable.unwrap();

        let mut difference = closest_chaseable.0.translation - quakka.0.translation;
        difference = difference.normalize();

        let in_attack_distance = quakka
            .0
            .translation
            .distance(closest_chaseable.0.translation)
            < QUAKKA_HIT_DISTANCE;
        if in_attack_distance {
            if quakka.1.cooldown.finished() {
                quakka.1.cooldown.reset();
                closest_chaseable.2.current_health -= quakka.1.damage;
            }
        } else {
            quakka.0.translation += (difference) * time.delta_secs() * QUAKKA_SPEED;
        }
    }
}

fn update_healthbars(
    mut commands: Commands,
    mut healthbar_q: Query<(Entity, &Parent), With<HealthBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    health: Query<&Health>,
) {
    for (healthbar, troop) in healthbar_q.iter_mut() {
        let health = health.get(troop.get());
        if health.is_err() {
            panic!("Health component not on troop!");
        }

        let health = health.unwrap();
        let health_percentage = health.current_health / health.max_health;

        commands.entity(healthbar).insert(Mesh2d(
            meshes.add(Rectangle::new(health_percentage * 100.0, 10.0)),
        ));

        commands
            .entity(healthbar)
            .insert_if_new(MeshMaterial2d(materials.add(Color::from(RED))));
    }
}

fn spawn_entities(asset_server: Res<AssetServer>, mut commands: Commands) {
    let quakka = commands
        .spawn((
            Sprite {
                image: asset_server.load("quakka.png"),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(0., 200., 0.),
                ..default()
            },
            Health {
                current_health: 100.0,
                max_health: 100.0,
            },
            Quakka,
            Attacker {
                cooldown: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
                damage: QUAKKA_DAMAGE,
            },
        ))
        .id();

    add_healthbar_child(quakka, 60., &mut commands);

    let farmer = commands.spawn((
        Sprite {
            image: asset_server.load("farmer.png"),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 0., 0.),
            ..default()
        },
        Farmer,
        Chaseable,
        Health {
            current_health: 100.0,
            max_health: 100.0,
        },
    )).id();
    
    add_healthbar_child(farmer, 30., &mut commands);
}

fn add_healthbar_child(e: Entity, height: f32, commands: &mut Commands) {
    let healthbar = commands
        .spawn((Transform::from_xyz(0., height, 1.), HealthBar))
        .id();

    commands.entity(e).add_children(&[healthbar]);
}
