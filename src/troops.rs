use bevy::{
    color::palettes::css::*,
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use crate::{
    deckbar::{SelectedCard, Troop},
    global::*,
};

#[derive(Component)]
pub struct Quakka;

#[derive(Component)]
pub struct Attacker {
    pub cooldown: Timer,
    pub damage: f32,
}

#[derive(Component)]
#[require(Chaseable)]
pub struct Nest;

#[derive(Component, Default)]
struct Chaseable;

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
struct Farmer;

#[derive(Component)]
struct GoingToBridge;

#[derive(Component)]
pub struct Bridge;

const QUAKKA_SPEED: f32 = 75.0;
const QUAKKA_HIT_DISTANCE: f32 = 50.0;
pub const QUAKKA_DAMAGE: f32 = 60.0;

const FARMER_SPEED: f32 = 25.0;

pub fn troops(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            (
                quakka_chase_and_attack,
                delete_dead_entities,
                update_healthbars,
            )
                .chain(),
            intialize_healthbar,
            farmer_go_to_bridge,
            farmer_go_up,
            spawn_troop_on_click,
            tick_attacker_cooldowns,
        )
            .run_if(in_state(GameState::InGame)),
    );
}

fn intialize_healthbar(q: Query<(Entity, &Health), Added<Health>>, mut commands: Commands) {
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

fn quakka_chase_and_attack(
    mut quakkas: Query<(&mut Transform, &mut Attacker), (With<Quakka>, Without<Nest>)>,
    mut chaseables: Query<(&Transform, Entity, &mut Health), (With<Chaseable>, Without<Quakka>)>,
    time: Res<Time>,
) {
    for mut quakka in quakkas.iter_mut() {
        let closest_chaseable = chaseables.iter_mut().max_by(|a, b| {
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
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mouse_coords: Res<CursorWorldCoords>,
    selected_card: Res<SelectedCard>,
) {
    for ev in mousebtn_evr.read() {
        if ev.state != ButtonState::Pressed {
            return;
        }

        if let Some((_, troop)) = selected_card.0 {
            match troop {
                Troop::Farmer => {
                    commands.spawn((
                        Sprite {
                            image: asset_server.load("farmer.png"),
                            custom_size: Some(Vec2::new(30.0, 30.0)),
                            ..default()
                        },
                        Transform {
                            translation: mouse_coords.0.extend(0.),
                            ..default()
                        },
                        Farmer,
                        GoingToBridge,
                    ));
                }
            }
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
