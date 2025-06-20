use crate::global::IsPointerOverUi;
use crate::manage_level::IsPaused;
use crate::manage_level::LevelEntity;
use crate::{
    deckbar::{DeleteSelectedCard, SelectedCard},
    global::{CursorWorldCoords, GameState},
};

use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::{color::palettes::css::*, prelude::*};
use debug::debug;
use farmer::farmer_go_to_bridge;
use farmer::farmer_go_to_exit;
use farmer::farmer_plugin;
use farmer::kill_farmer_reaching_exit;
use farmer::GoingToBridge;
use nest::nest_plugin;
use nest::nest_shoot;

#[derive(Component)]
#[require(LevelEntity, NestTarget, WaterballTarget)]
#[require(SpawnedCard(Card::Quakka))]
pub struct Quakka;

#[derive(Component)]
#[require(QuakkaTarget, LevelEntity, GoingToBridge)]
#[require(SpawnedCard(Card::Farmer))]
pub struct Farmer;

#[derive(Component)]
#[require(SpawnedCard(Card::Waterball))]
pub struct Waterball {
    pub radius: f32,
}

#[derive(Component, Default)]
#[require(QuakkaTarget, LevelEntity)]
#[require(SpawnedCard(Card::Nest))]
pub struct Nest {
    current_victim: Option<Entity>,
}

#[derive(Component, DerefMut, Deref)]
pub struct SpawnedCard(Card);

#[derive(Component, Default)]
pub struct WaterballTarget;

#[derive(Component)]
pub struct Attacker {
    pub cooldown: Timer,
}

#[derive(Component, Default)]
pub struct QuakkaTarget;

#[derive(Component, Default)]
pub struct NestTarget;

#[derive(Component)]
#[component(on_add = initialize_healthbar)]
pub struct Health {
    pub current_health: f32,
    pub max_health: f32,
    pub healthbar_height: f32,
}

#[derive(Component)]
#[require(Transform)]
struct HealthBar;

#[derive(Event, Deref, DerefMut)]
pub struct CardDeath(Card);

pub fn card_behaviors(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            (
                farmer_go_to_bridge,
                farmer_go_to_exit,
                kill_farmer_reaching_exit,
                tick_attacker_cooldowns,
                quakka_chase_and_attack,
                explode_waterballs,
                nest_shoot,
            )
                .run_if(in_state(IsPaused::False)),
            spawn_card_on_click,
            delete_dead_entities,
            update_healthbars,
        )
            .run_if(in_state(GameState::InGame)),
    )
    .add_event::<CardDeath>()
    .add_plugins(nest_plugin)
    .add_plugins(farmer_plugin)
    .add_plugins(debug);
}

fn initialize_healthbar(mut world: DeferredWorld, context: HookContext) {
    let healthbar_height = world
        .get::<Health>(context.entity)
        .unwrap()
        .healthbar_height;

    let healthbar = world
        .commands()
        .spawn((Transform::from_xyz(0., healthbar_height, 1.), HealthBar))
        .id();

    world.commands().entity(context.entity).add_child(healthbar);
}

fn update_healthbars(
    mut commands: Commands,
    mut healthbar_q: Query<(Entity, &ChildOf), With<HealthBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    health: Query<&Health>,
) {
    for (healthbar, card) in healthbar_q.iter_mut() {
        let health = health.get(card.parent());
        if health.is_err() {
            panic!("Health component not on card!");
        }

        let health = health.unwrap();
        let health_percentage = health.current_health / health.max_health;

        commands.entity(healthbar).try_insert(Mesh2d(
            meshes.add(Rectangle::new(health_percentage * 100.0, 10.0)),
        ));

        commands
            .entity(healthbar)
            .try_insert_if_new(MeshMaterial2d(materials.add(Color::from(RED))));
    }
}

fn quakka_chase_and_attack(
    mut quakkas: Query<(&mut Transform, &mut Attacker), With<Quakka>>,
    mut quakka_targets: Query<
        (&Transform, Entity, &mut Health),
        (With<QuakkaTarget>, Without<Quakka>),
    >,
    time: Res<Time>,

    card_consts: Res<CardConsts>,
) {
    for mut quakka in quakkas.iter_mut() {
        let closest_chaseable = quakka_targets.iter_mut().min_by(|a, b| {
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

        let in_attack_distance = distance_to_chaseable < card_consts.quakka.range;

        if in_attack_distance && quakka.1.cooldown.finished() {
            quakka.1.cooldown.reset();
            closest_chaseable.2.current_health -= card_consts.quakka.damage;
        } else if !in_attack_distance {
            let mut to_chaseable = closest_chaseable.0.translation - quakka.0.translation;
            to_chaseable = to_chaseable.normalize();

            quakka.0.translation += to_chaseable * time.delta_secs() * card_consts.quakka.speed;
        }
    }
}

fn explode_waterballs(
    mut waterball_targets: Query<Entity, With<WaterballTarget>>,
    waterballs: Query<(Entity, &Waterball)>,
    mut health_q: Query<&mut Health>,
    mut attacker_q: Query<&mut Attacker, With<Waterball>>,
    transform_q: Query<&Transform>,

    mut commands: Commands,

    card_consts: Res<CardConsts>,
) {
    for (waterball_e, waterball) in waterballs {
        let waterball_attacker = attacker_q.get_mut(waterball_e).unwrap();

        if !waterball_attacker.cooldown.finished() {
            continue;
        }

        for target in &mut waterball_targets {
            let target_transform = transform_q.get(target);

            // Checking if within distance
            if let Ok(target_transform) = target_transform {
                let target_position = target_transform.translation.truncate();
                let waterball_position =
                    transform_q.get(waterball_e).unwrap().translation.truncate();

                let is_in_explosion_distance =
                    waterball_position.distance(target_position) < waterball.radius;
                if !is_in_explosion_distance {
                    continue;
                }
            }

            let target_health = health_q.get_mut(target);
            if let Ok(mut target_health) = target_health {
                target_health.current_health -= card_consts.waterball.damage;
            }
        }

        commands.entity(waterball_e).despawn();
    }
}

fn delete_dead_entities(
    healths: Query<(&Health, Entity)>,
    mut commands: Commands,
    spawned_card_q: Query<&SpawnedCard>,
    mut card_destroyed_ev: EventWriter<CardDeath>,
) {
    for (health, e) in healths.iter() {
        if health.current_health <= 0.0 {
            commands.entity(e).despawn();
            card_destroyed_ev.write(CardDeath(**spawned_card_q.get(e).unwrap()));
        }
    }
}

mod farmer {
    use super::{Farmer, Health};
    use crate::{
        card::CardConsts,
        global::{BRIDGE_LOCATIONS, FARMER_EXIT_LOCATION},
    };
    use bevy::prelude::*;

    #[derive(Component, Default)]
    pub struct GoingToBridge;

    #[derive(Component)]
    pub struct Bridge;

    #[derive(Component)]
    pub struct Exit;

    pub fn farmer_plugin(app: &mut App) {
        app.add_systems(Startup, (spawn_bridge_locations, spawn_exit));
    }

    pub(crate) fn farmer_go_to_bridge(
        mut farmers: Query<
            (&mut Transform, Entity),
            (With<Farmer>, With<GoingToBridge>, Without<Bridge>),
        >,
        bridges: Query<&Transform, (With<Bridge>, Without<Farmer>)>,
        mut commands: Commands,
        time: Res<Time>,

        card_consts: Res<CardConsts>,
    ) {
        for farmer in farmers.iter_mut() {
            let (mut farmer_transform, farmer_e) = farmer;
            let farmer_translation = farmer_transform.translation;
            let bridge = bridges.iter().max_by(|a, b| {
                let a_distance = farmer_translation.distance(a.translation);
                let b_distance = farmer_translation.distance(b.translation);
                b_distance.partial_cmp(&a_distance).unwrap()
            });

            if bridge.is_none() {
                warn!("No bridge found for farmer");
                return;
            }

            let bridge = bridge.unwrap();

            let mut difference = bridge.translation - farmer_translation;

            difference = difference.normalize();

            if farmer_translation.distance(bridge.translation) < 10.0 {
                commands.entity(farmer_e).remove::<GoingToBridge>();
            } else {
                farmer_transform.translation +=
                    (difference) * time.delta_secs() * card_consts.farmer.speed;
            }
        }
    }

    pub fn farmer_go_to_exit(
        mut farmer_q: Query<&mut Transform, (With<Farmer>, Without<GoingToBridge>)>,
        exit: Single<&Transform, (With<Exit>, Without<Farmer>)>,
        time: Res<Time>,

        card_consts: Res<CardConsts>,
    ) {
        for mut farmer in farmer_q.iter_mut() {
            let mut delta = exit.translation - farmer.translation;
            delta = delta.normalize_or_zero();

            farmer.translation += delta * time.delta_secs() * card_consts.farmer.speed;
        }
    }

    pub fn kill_farmer_reaching_exit(
        mut farmer_q: Query<(&mut Health, &Transform), With<Farmer>>,
        exit: Single<&Transform, (With<Exit>, Without<Farmer>)>,
    ) {
        for (mut farmer_health, farmer_transform) in farmer_q.iter_mut() {
            if farmer_transform.translation.distance(exit.translation) < 10.0 {
                farmer_health.current_health = 0.0;
            };
        }
    }

    fn spawn_bridge_locations(mut commands: Commands) {
        commands.spawn((
            Bridge,
            Transform {
                translation: BRIDGE_LOCATIONS.0.extend(0.0),
                ..default()
            },
        ));

        commands.spawn((
            Bridge,
            Transform {
                translation: BRIDGE_LOCATIONS.1.extend(0.0),
                ..default()
            },
        ));
    }

    fn spawn_exit(mut commands: Commands) {
        commands.spawn((
            Exit,
            Transform {
                translation: Vec2::from(FARMER_EXIT_LOCATION).extend(0.0),
                ..default()
            },
        ));
    }
}

fn spawn_card_on_click(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mouse_coords: Res<CursorWorldCoords>,
    is_pointer_over_ui: Res<IsPointerOverUi>,
    selected_card: Option<Single<&MaybeCard, With<SelectedCard>>>,
) {
    let Some(selected_card) = selected_card.map(Single::into_inner) else {
        mousebtn_evr.clear();
        return;
    };

    let Some(selected_card) = selected_card.0 else {
        return;
    };

    for ev in mousebtn_evr.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }

        if !is_pointer_over_ui.0 {
            commands.queue(SpawnCard::new(selected_card, mouse_coords.0));
            commands.queue(DeleteSelectedCard::default());
        }
    }
}

fn tick_attacker_cooldowns(mut attackers: Query<&mut Attacker>, time: Res<Time>) {
    for mut attacker in attackers.iter_mut() {
        if attacker.cooldown.mode() == TimerMode::Repeating {
            panic!("Attack cooldown should be once");
        }
        attacker.cooldown.tick(time.delta());
    }
}

mod nest {
    use super::{Attacker, Health, Nest, NestTarget};
    use crate::card::CardConsts;
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Egg {
        from_nest: Entity,
    }

    pub fn nest_plugin(app: &mut App) {
        app.add_systems(FixedUpdate, (spawn_eggs, render_eggs));
    }

    pub fn nest_shoot(
        mut victims: Query<(&Transform, &mut Health, Entity), With<NestTarget>>,
        nests: Query<(&Transform, &mut Attacker, &mut Nest), With<Nest>>,

        card_consts: Res<CardConsts>,
    ) {
        for mut nest in nests {
            let closest_victim = victims.iter_mut().min_by(|a, b| {
                let a_dist = nest.0.translation.distance(a.0.translation);
                let b_dist = nest.0.translation.distance(b.0.translation);
                a_dist.total_cmp(&b_dist)
            });

            if closest_victim.is_none() {
                nest.2.current_victim = None;
                continue;
            }

            let mut closest_victim = closest_victim.unwrap();

            let dist_to_victim = nest.0.translation.distance(closest_victim.0.translation);

            if dist_to_victim < card_consts.nest.range {
                nest.2.current_victim = Some(closest_victim.2);

                if nest.1.cooldown.finished() {
                    nest.1.cooldown.reset();
                    closest_victim.1.current_health -= card_consts.nest.damage;
                }
            } else {
                nest.2.current_victim = None;
            }
        }
    }

    fn spawn_eggs(
        mut commands: Commands,
        nest_q: Query<Entity, Added<Nest>>,
        asset_server: Res<AssetServer>,
    ) {
        for nest in nest_q {
            const IMAGE_SIZE: Vec2 = Vec2::new(50.0, 65.0);
            commands.spawn((
                Egg { from_nest: nest },
                Sprite {
                    image: asset_server.load("nest-egg.png"),
                    custom_size: Some(IMAGE_SIZE * 0.5),
                    color: Color::NONE,
                    ..default()
                },
            ));
        }
    }

    pub fn render_eggs(
        mut commands: Commands,
        nest_q: Query<&Nest>,
        attacker_q: Query<&Attacker, With<Nest>>,
        transform_q: Query<&mut Transform>,
        eggs: Query<(Entity, &Egg, &mut Sprite)>,
    ) {
        for mut egg in eggs {
            let nest = nest_q.get(egg.1.from_nest);

            // Nest must have died
            if nest.is_err() {
                commands.entity(egg.0).despawn();
                return;
            }

            let nest = nest.unwrap();

            if let Some(victim) = nest.current_victim {
                egg.2.color = Color::WHITE;

                let victim_transform = transform_q.get(victim);
                let nest_transform = transform_q.get(egg.1.from_nest);
                let nest_attack = attacker_q.get(egg.1.from_nest);

                // Victim might have died
                if let (Ok(victim_transform), Ok(nest_transform), Ok(nest_attack)) =
                    (victim_transform, nest_transform, nest_attack)
                {
                    let nest_to_victim: Vec3 =
                        victim_transform.translation - nest_transform.translation;

                    commands.entity(egg.0).insert(Transform::from_translation(
                        nest_transform.translation
                            + (nest_to_victim * nest_attack.cooldown.fraction()),
                    ));
                }
            } else {
                egg.2.color = Color::NONE;
            }
        }
    }
}

pub use debug::IsSpawnedCardDebugOverlayEnabled;

use super::Card;
use super::CardConsts;
use super::MaybeCard;
use super::SpawnCard;

mod debug {
    use crate::{card::CardConsts, debug::in_debug};

    use super::{farmer::Bridge, Nest};
    use bevy::{color::palettes::tailwind::PINK_600, prelude::*};

    #[derive(Resource, PartialEq)]
    pub struct IsSpawnedCardDebugOverlayEnabled(pub bool);

    impl Default for IsSpawnedCardDebugOverlayEnabled {
        fn default() -> Self {
            IsSpawnedCardDebugOverlayEnabled(false)
        }
    }

    pub fn debug(app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (show_bridge_points, show_nest_attack_radius)
                .run_if(in_debug.and(resource_equals(IsSpawnedCardDebugOverlayEnabled(true)))),
        )
        .init_resource::<IsSpawnedCardDebugOverlayEnabled>();
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

    fn show_nest_attack_radius(
        mut draw: Gizmos,
        nests: Query<&Transform, With<Nest>>,
        card_consts: Res<CardConsts>,
    ) {
        for nest in nests {
            draw.circle_2d(
                Isometry2d::from_translation(nest.translation.truncate()),
                card_consts.nest.range,
                PINK_600,
            );
        }
    }
}
