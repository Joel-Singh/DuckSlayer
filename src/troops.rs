use bevy::{color::palettes::css::*, prelude::*};
use debug::debug;
use nest::nest_shoot;

pub use nest::spawn_nest;

use crate::{
    deckbar::{DeleteSelectedCard, SelectedCard, Troop},
    global::*,
};

#[derive(Component)]
pub struct Quakka;

#[derive(Component)]
pub struct Attacker {
    pub cooldown: Timer,
    pub damage: f32,
}

#[derive(Component, Default)]
pub struct Chaseable;

#[derive(Component)]
pub struct Health {
    pub current_health: f32,
    pub max_health: f32,
    pub healthbar_height: f32,
}

#[derive(Component)]
#[require(Transform)]
struct HealthBar;

#[derive(Component)]
#[require(Chaseable)]
pub struct Farmer;

#[derive(Component)]
struct GoingToBridge;

#[derive(Component)]
pub struct Bridge;

#[derive(Component)]
pub struct Arena;

const QUAKKA_SPEED: f32 = 75.0;
const QUAKKA_HIT_DISTANCE: f32 = 50.0;
pub const QUAKKA_DAMAGE: f32 = 60.0;

const FARMER_SPEED: f32 = 25.0;

pub fn troops(app: &mut App) {
    app.add_systems(Startup, spawn_arena_area)
        .add_systems(
            FixedUpdate,
            (
                delete_dead_entities.run_if(in_state(IsPaused::False)),
                update_healthbars,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (
                farmer_go_to_bridge,
                farmer_go_up,
                spawn_troop_on_click,
                tick_attacker_cooldowns,
                quakka_chase_and_attack,
                nest_shoot,
            )
                .run_if(in_state(GameState::InGame).and(in_state(IsPaused::False))),
        )
        .add_systems(
            FixedUpdate,
            initialize_healthbar.run_if(in_state(GameState::InGame)),
        )
        .add_plugins(debug);
}

fn initialize_healthbar(q: Query<(Entity, &Health), Added<Health>>, mut commands: Commands) {
    for (entity, health) in &q {
        let healthbar = commands
            .spawn((
                Transform::from_xyz(0., health.healthbar_height, 1.),
                HealthBar,
            ))
            .id();

        commands.entity(entity).add_children(&[healthbar]);
    }
}

fn spawn_arena_area(mut commands: Commands) {
    commands.spawn((
        Arena,
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            ..default()
        },
        Button,
        GlobalZIndex(-1),
    ));
}

fn quakka_chase_and_attack(
    mut quakkas: Query<(&mut Transform, &mut Attacker), With<Quakka>>,
    mut chaseables: Query<(&Transform, Entity, &mut Health), (With<Chaseable>, Without<Quakka>)>,
    time: Res<Time>,
) {
    for mut quakka in quakkas.iter_mut() {
        let closest_chaseable = chaseables.iter_mut().min_by(|a, b| {
            let a_distance = quakka.0.translation.distance(a.0.translation);
            let b_distance = quakka.0.translation.distance(b.0.translation);
            a_distance.partial_cmp(&b_distance).unwrap()
        });

        if closest_chaseable.is_none() {
            continue;
        }

        let mut closest_chaseable = closest_chaseable.unwrap();

        let distance_to_chaseable = quakka
            .0
            .translation
            .distance(closest_chaseable.0.translation);

        let in_attack_distance = distance_to_chaseable < QUAKKA_HIT_DISTANCE;

        if in_attack_distance && quakka.1.cooldown.finished() {
            quakka.1.cooldown.reset();
            closest_chaseable.2.current_health -= quakka.1.damage;
        } else if !in_attack_distance {
            let mut to_chaseable = closest_chaseable.0.translation - quakka.0.translation;
            to_chaseable = to_chaseable.normalize();

            quakka.0.translation += to_chaseable * time.delta_secs() * QUAKKA_SPEED;
        }
    }
}

fn delete_dead_entities(healths: Query<(&Health, Entity)>, mut commands: Commands) {
    for (health, e) in healths.iter() {
        if health.current_health <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

fn update_healthbars(
    mut commands: Commands,
    mut healthbar_q: Query<(Entity, &ChildOf), With<HealthBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    health: Query<&Health>,
) {
    for (healthbar, troop) in healthbar_q.iter_mut() {
        let health = health.get(troop.parent());
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

fn farmer_go_to_bridge(
    mut farmers: Query<
        (&mut Transform, Entity),
        (With<Farmer>, With<GoingToBridge>, Without<Bridge>),
    >,
    bridges: Query<&Transform, (With<Bridge>, Without<Farmer>)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for farmer in farmers.iter_mut() {
        let (mut farmer_transform, farmer_e) = farmer;
        let farmer_translation = farmer_transform.translation;
        let bridge = bridges
            .iter()
            .max_by(|a, b| {
                let a_distance = farmer_translation.distance(a.translation);
                let b_distance = farmer_translation.distance(b.translation);
                b_distance.partial_cmp(&a_distance).unwrap()
            })
            .unwrap();

        let mut difference = bridge.translation - farmer_translation;

        difference = difference.normalize();

        if farmer_translation.distance(bridge.translation) < 10.0 {
            commands.entity(farmer_e).remove::<GoingToBridge>();
        } else {
            farmer_transform.translation += (difference) * time.delta_secs() * FARMER_SPEED;
        }
    }
}

fn farmer_go_up(
    mut farmer_transforms: Query<&mut Transform, (With<Farmer>, Without<GoingToBridge>)>,
    time: Res<Time>,
) {
    for mut farmer_transform in farmer_transforms.iter_mut() {
        farmer_transform.translation.y += time.delta_secs() * FARMER_SPEED;
    }
}

fn spawn_troop_on_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interaction_q: Query<&Interaction, (With<Arena>, Changed<Interaction>)>,
    mouse_coords: Res<CursorWorldCoords>,
    selected_card: Res<SelectedCard>,
) {
    for interaction in interaction_q {
        if *interaction != Interaction::Pressed {
            return;
        }

        if let Some((_, troop)) = selected_card.0 {
            match troop {
                Troop::Farmer => {
                    commands.spawn((
                        Sprite {
                            image: asset_server.load("farmer.png"),
                            custom_size: Some(FARMER_SIZE),
                            ..default()
                        },
                        Transform {
                            translation: mouse_coords.0.extend(0.),
                            ..default()
                        },
                        Farmer,
                        GoingToBridge,
                        Chaseable,
                        Health {
                            current_health: 100.0,
                            max_health: 100.0,
                            healthbar_height: 60.,
                        },
                    ));
                }
            }

            commands.queue(DeleteSelectedCard::default());
        }
    }
}

fn tick_attacker_cooldowns(mut attackers: Query<&mut Attacker>, time: Res<Time>) {
    for mut attacker in attackers.iter_mut() {
        if attacker.cooldown.mode() == TimerMode::Repeating {
            panic!("Attack coolodwn should be once");
        }
        attacker.cooldown.tick(time.delta());
    }
}

mod nest {
    use super::{Attacker, Chaseable, Farmer, Health};
    use crate::global::{NEST_ATTACK_DISTANCE, NEST_DAMAGE};
    use bevy::prelude::*;
    use std::time::Duration;

    #[derive(Component)]
    #[require(Chaseable)]
    pub struct Nest;

    pub fn spawn_nest(translation: Vec3, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn((
            Sprite {
                image: asset_server.load("nest.png"),
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            Transform {
                translation,
                ..default()
            },
            Health {
                current_health: 100.0,
                max_health: 100.0,
                healthbar_height: 60.,
            },
            Nest,
            Attacker {
                cooldown: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
                damage: NEST_DAMAGE,
            },
        ));
    }

    pub fn nest_shoot(
        mut victims: Query<(&Transform, &mut Health), (Without<Nest>, Without<Farmer>)>,
        nests: Query<(&Transform, &mut Attacker), With<Nest>>,
    ) {
        for mut nest in nests {
            let closest_victim = victims.iter_mut().min_by(|a, b| {
                let a_dist = nest.0.translation.distance(a.0.translation);
                let b_dist = nest.0.translation.distance(b.0.translation);
                a_dist.total_cmp(&b_dist)
            });

            if closest_victim.is_none() {
                continue;
            }

            let mut closest_victim = closest_victim.unwrap();

            let dist_to_victim = nest.0.translation.distance(closest_victim.0.translation);

            if dist_to_victim < NEST_ATTACK_DISTANCE && nest.1.cooldown.finished() {
                nest.1.cooldown.reset();
                closest_victim.1.current_health -= NEST_DAMAGE;
            }
        }
    }
}

mod debug {
    use crate::global::{IsDebug, NEST_ATTACK_DISTANCE};

    use super::{nest::Nest, Bridge};
    use bevy::{color::palettes::tailwind::PINK_600, prelude::*};

    pub fn debug(app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (show_bridge_points, show_nest_attack_radius).run_if(resource_equals(IsDebug(true))),
        );
    }

    fn show_bridge_points(mut draw: Gizmos, bridges: Query<&Transform, With<Bridge>>) {
        for bridge in bridges {
            draw.circle_2d(
                Isometry2d::from_translation(bridge.translation.truncate()),
                10.,
                PINK_600,
            );
        }
    }

    fn show_nest_attack_radius(mut draw: Gizmos, nests: Query<&Transform, With<Nest>>) {
        for nest in nests {
            draw.circle_2d(
                Isometry2d::from_translation(nest.translation.truncate()),
                NEST_ATTACK_DISTANCE,
                PINK_600,
            );
        }
    }
}
