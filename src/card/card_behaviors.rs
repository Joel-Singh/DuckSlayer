mod follow_path;

use std::time::Duration;

use crate::global::GameState;
use crate::global::HEALTHBAR_SIZE;
use crate::manage_level::IsPaused;
use crate::{card::Card, manage_level::LevelEntity};
use attacker::attacker_plugin;
pub use attacker::Attacker;
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use farmer::farmer_plugin;
use farmer::kill_farmer_reaching_exit;
use follow_path::follow_path_plugin;
use nest::nest_plugin;
use quakka::quakka_plugin;
pub use quakka::Quakka;
use walk_animation::walk_animation_plugin;
use walk_animation::WalkAnim;

use super::CardConsts;

#[derive(Component, DerefMut, Deref)]
pub struct SpawnedCard(Card);

#[derive(Component)]
#[require(LevelEntity, WalkAnim)]
#[require(SpawnedCard(Card::Farmer))]
pub struct Farmer;

#[derive(Component)]
#[require(SpawnedCard(Card::Waterball))]
pub struct Waterball {
    pub radius: f32,
    pub timer: Timer,
}

impl Waterball {
    pub fn new(radius: f32) -> Waterball {
        Waterball {
            radius,
            timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
        }
    }
}

#[derive(Component)]
#[require(LevelEntity)]
#[require(SpawnedCard(Card::Nest))]
pub struct Nest;

#[derive(Component, Default)]
pub struct WaterballTarget;

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
                kill_farmer_reaching_exit,
                explode_waterballs,
                tick_waterball_timers,
            )
                .run_if(in_state(IsPaused::False)),
            delete_dead_entities,
            update_healthbars,
        )
            .run_if(in_state(GameState::InGame)),
    )
    .add_event::<CardDeath>()
    .add_plugins(nest_plugin)
    .add_plugins(farmer_plugin)
    .add_plugins(attacker_plugin)
    .add_plugins(quakka_plugin)
    .add_plugins(walk_animation_plugin)
    .add_plugins(follow_path_plugin);
}

fn initialize_healthbar(mut world: DeferredWorld, context: HookContext) {
    let asset_server = world.resource::<AssetServer>();
    let healthbar_sprite: Handle<Image> = asset_server.load("healthbar.png");

    let healthbar_height = world
        .get::<Health>(context.entity)
        .unwrap()
        .healthbar_height;

    let mut commands = world.commands();

    let healthbar = commands
        .spawn((
            Transform::from_xyz(0., healthbar_height, 1.),
            HealthBar,
            Sprite {
                rect: Some(Rect::from_corners(Vec2::ZERO, HEALTHBAR_SIZE.into())),
                image: healthbar_sprite,
                ..default()
            },
        ))
        .id();

    world.commands().entity(context.entity).add_child(healthbar);
}

fn update_healthbars(
    mut healthbar_q: Query<(&mut Sprite, &ChildOf), With<HealthBar>>,
    health: Query<&Health>,
) {
    for (mut healthbar_sprite, card) in healthbar_q.iter_mut() {
        let health = health.get(card.parent());
        if health.is_err() {
            panic!("Health component not on card!");
        }

        let health = health.unwrap();
        let health_percentage = health.current_health / health.max_health;

        healthbar_sprite.rect = Some(Rect::from_corners(
            Vec2::ZERO,
            (HEALTHBAR_SIZE.0 * health_percentage, HEALTHBAR_SIZE.1).into(),
        ));
    }
}

fn tick_waterball_timers(waterballs: Query<&mut Waterball>, time: Res<Time>) {
    for mut waterball in waterballs {
        waterball.timer.tick(time.delta());
    }
}

fn explode_waterballs(
    mut waterball_targets: Query<Entity, With<WaterballTarget>>,
    waterballs: Query<(Entity, &Waterball)>,
    mut health_q: Query<&mut Health>,
    transform_q: Query<&Transform>,

    mut commands: Commands,

    card_consts: Res<CardConsts>,
) {
    for (waterball_e, waterball) in waterballs {
        if !waterball.timer.finished() {
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

mod attacker {
    use std::time::Duration;

    use bevy::{color::palettes::css::RED, prelude::*};

    use crate::{card::Card, global::GameState, manage_level::IsPaused};

    use super::{Health, SpawnedCard};

    pub fn attacker_plugin(app: &mut App) {
        app.add_systems(
            FixedUpdate,
            attackers_attack.run_if(in_state(GameState::InGame).and(in_state(IsPaused::False))),
        );

        if crate::debug::get_debug_env_var() {
            app.add_systems(FixedUpdate, display_range);
        }
    }

    #[derive(Component)]
    pub struct Attacker {
        damage: f32,
        range: f32,
        cooldown: Timer,
        prey: Vec<Card>,
        current_victim: Option<Entity>,
        current_victim_in_range: bool,
        current_victim_in_range_fraction: Option<f32>, // from 0. to 1.
    }

    impl Attacker {
        pub fn current_victim(&self) -> Option<Entity> {
            self.current_victim
        }

        pub fn current_victim_in_range(&self) -> bool {
            self.current_victim_in_range
        }

        pub fn current_victim_in_range_fraction(&self) -> Option<f32> {
            self.current_victim_in_range_fraction
        }

        pub fn cooldown_fraction(&self) -> f32 {
            self.cooldown.fraction()
        }

        pub fn new(
            damage: f32,
            range: f32,
            prey: Vec<Card>,
            attack_cooldown: Duration,
        ) -> Attacker {
            Attacker {
                damage,
                range,
                prey,
                cooldown: Timer::new(attack_cooldown, TimerMode::Once),
                current_victim: None,
                current_victim_in_range: false,
                current_victim_in_range_fraction: None, // 1.0 is farthest, 0.5 is closest
            }
        }
    }

    fn attackers_attack(
        mut possible_targets: Query<(Entity, &mut Health, &SpawnedCard), With<Transform>>,
        attackers: Query<(Entity, &mut Attacker)>,
        transform_q: Query<&Transform>,

        time: Res<Time>,
    ) {
        for (attacker_e, mut attacker) in attackers {
            let attacker_translation = transform_q.get(attacker_e).unwrap().translation;

            let closest_target = possible_targets
                .iter_mut()
                .filter(|t| attacker.prey.contains(&**t.2) && t.0 != attacker_e)
                .min_by(|a, b| {
                    let a = transform_q.get(a.0).unwrap().translation;
                    let b = transform_q.get(b.0).unwrap().translation;

                    let a_dist = attacker_translation.distance(a);
                    let b_dist = attacker_translation.distance(b);

                    a_dist.partial_cmp(&b_dist).unwrap()
                });

            let Some(mut closest_target) = closest_target else {
                attacker.current_victim = None;
                attacker.current_victim_in_range = false;
                attacker.current_victim_in_range_fraction = None;
                attacker.cooldown.reset();
                return;
            };
            attacker.current_victim = Some(closest_target.0);

            let closest_target_translation = transform_q.get(closest_target.0).unwrap().translation;
            let dist_to_target = attacker_translation.distance(closest_target_translation);
            let in_attack_dist = dist_to_target < attacker.range;

            if in_attack_dist {
                attacker.cooldown.tick(time.delta());
                attacker.current_victim_in_range = true;
                attacker.current_victim_in_range_fraction = Some(dist_to_target / attacker.range);
                if attacker.cooldown.finished() {
                    closest_target.1.current_health -= attacker.damage;
                    attacker.cooldown.reset();
                }
            } else {
                attacker.cooldown.reset();
                attacker.current_victim_in_range = false;
                attacker.current_victim_in_range_fraction = None;
            }
        }
    }

    fn display_range(attackers: Query<(&Transform, &mut Attacker)>, mut draw: Gizmos) {
        for (transform, attacker) in attackers {
            draw.circle_2d(
                Isometry2d::from_translation(transform.translation.truncate()),
                attacker.range,
                RED,
            );
        }
    }
}

mod walk_animation {
    use bevy::prelude::*;
    use std::f32::consts::PI;

    use crate::manage_level::IsPaused;

    #[derive(Component, Default)]
    pub struct WalkAnim {
        pub progress: f32, // From 0.0 to 1.0
    }

    #[derive(Component)]
    pub struct CancelWalkAnim;

    pub fn walk_animation_plugin(app: &mut App) {
        app.add_systems(
            Update,
            (animate_walking, remove_stray_cancels).run_if(in_state(IsPaused::False)),
        );
    }

    fn remove_stray_cancels(
        strays: Query<Entity, (With<CancelWalkAnim>, Without<WalkAnim>)>,
        mut commands: Commands,
    ) {
        for stray in strays {
            commands.entity(stray).try_remove::<CancelWalkAnim>();
        }
    }

    fn animate_walking(
        walkers: Query<(&mut Transform, &mut WalkAnim, Has<CancelWalkAnim>, Entity)>,
        time: Res<Time<Real>>,

        mut commands: Commands,
    ) {
        const ANIM_SPEED: f32 = 4.;
        const LENGTH: f32 = 1. / 40.;

        let easing_curve = EasingCurve::new(0., 2. * PI * LENGTH, EaseFunction::CubicInOut);

        for (mut transform, mut walk_anim, is_canceling, e) in walkers {
            walk_anim.progress += time.delta_secs();

            let curve_sample = wrap_around(0., 1., walk_anim.progress * ANIM_SPEED);
            if is_canceling && curve_sample < 0.05 {
                commands.entity(e).try_remove::<CancelWalkAnim>();
                commands.entity(e).try_remove::<WalkAnim>();
            } else {
                transform.rotation =
                    Quat::from_rotation_z(easing_curve.sample(curve_sample).unwrap());
            }
        }

        fn wrap_around(start: f32, end: f32, x: f32) -> f32 {
            debug_assert!(start < end);
            let range = start.abs() + end.abs();

            let is_even = (x / range) as i32 % 2 == 0;

            let remainder = x % range;
            if is_even {
                start + remainder
            } else {
                end - remainder
            }
        }
    }
}

mod farmer {
    use crate::{card::CardConsts, global::FARMER_EXIT_LOCATION};

    use super::{follow_path::FollowPath, Farmer, Health};
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Exit;

    pub fn farmer_plugin(app: &mut App) {
        app.add_observer(farmers_go_to_exit);
    }

    pub fn farmers_go_to_exit(
        trigger: Trigger<OnAdd, Farmer>,
        mut commands: Commands,
        card_consts: Res<CardConsts>,
    ) -> () {
        commands.entity(trigger.target()).insert(FollowPath::new(
            FARMER_EXIT_LOCATION,
            card_consts.farmer.speed,
        ));
    }

    pub fn kill_farmer_reaching_exit(
        mut farmer_q: Query<(&mut Health, &Transform), With<Farmer>>,
        exit: Single<&Transform, (With<Exit>, Without<Farmer>)>,
    ) {
        for (mut farmer_health, farmer_transform) in farmer_q.iter_mut() {
            if farmer_transform.translation.distance(exit.translation) < 1.0 {
                farmer_health.current_health = 0.0;
            };
        }
    }
}

mod nest {
    use super::{Attacker, Nest};
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Egg {
        from_nest: Entity,
    }

    pub fn nest_plugin(app: &mut App) {
        app.add_systems(FixedUpdate, (spawn_eggs, render_eggs));
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
        attacker_q: Query<&Attacker, With<Nest>>,
        transform_q: Query<&mut Transform>,
        eggs: Query<(Entity, &Egg, &mut Sprite)>,
    ) {
        for mut egg in eggs {
            let Ok(nest_attack) = attacker_q.get(egg.1.from_nest) else {
                // Nest must have died
                commands.entity(egg.0).despawn();
                return;
            };

            if nest_attack.current_victim_in_range() {
                egg.2.color = Color::WHITE;

                let victim = nest_attack.current_victim().unwrap();

                let victim_transform = transform_q.get(victim);
                let nest_transform = transform_q.get(egg.1.from_nest);

                // Victim might have died
                if let (Ok(victim_transform), Ok(nest_transform)) =
                    (victim_transform, nest_transform)
                {
                    let nest_to_victim: Vec3 =
                        victim_transform.translation - nest_transform.translation;

                    commands.entity(egg.0).insert(Transform::from_translation(
                        nest_transform.translation
                            + (nest_to_victim * nest_attack.cooldown_fraction()),
                    ));
                }
            } else {
                egg.2.color = Color::NONE;
            }
        }
    }
}

mod quakka {
    use crate::{
        card::{card_behaviors::attacker, Card, CardConsts},
        global::GameState,
        manage_level::{IsPaused, LevelEntity},
    };
    use bevy::prelude::*;

    use super::{follow_path::FollowPath, Attacker, SpawnedCard, WaterballTarget};

    #[derive(Component)]
    #[require(LevelEntity, WaterballTarget)]
    #[require(SpawnedCard(Card::Quakka))]
    pub struct Quakka;

    pub fn quakka_plugin(app: &mut App) {
        app.add_systems(
            FixedUpdate,
            chase_current_victim.run_if(in_state(GameState::InGame).and(in_state(IsPaused::False))),
        );
    }

    fn chase_current_victim(
        quakkas: Query<(Entity, &Attacker, Option<&FollowPath>), With<Quakka>>,
        transform_q: Query<&Transform>,
        mut commands: Commands,

        card_consts: Res<CardConsts>,
    ) {
        const REGENERATE_PATH_TOLERANCE: f32 = 30.0;

        for (quakka_e, attacker, follow_path) in quakkas {
            if attacker.current_victim().is_none() {
                commands.entity(quakka_e).try_remove::<FollowPath>();
                continue;
            }

            if let Some(range_fraction) = attacker.current_victim_in_range_fraction() {
                if range_fraction >= 0.5 {
                    commands.entity(quakka_e).try_remove::<FollowPath>();
                    continue;
                }
            }

            if let Some(current_victim) = attacker.current_victim() {
                let Ok(current_victim_transform) = transform_q.get(current_victim) else {
                    return;
                };

                let mut generate_new_path = || {
                    commands.entity(quakka_e).insert(FollowPath::new(
                        (
                            current_victim_transform.translation.x as i32,
                            current_victim_transform.translation.y as i32,
                        ),
                        card_consts.quakka.speed,
                    ));
                };

                if let Some(follow_path) = follow_path {
                    let goal_dist_to_victim = current_victim_transform
                        .translation
                        .truncate()
                        .distance(Vec2::new(
                            follow_path.get_goal().0 as f32,
                            follow_path.get_goal().1 as f32,
                        ));

                    if goal_dist_to_victim > REGENERATE_PATH_TOLERANCE {
                        generate_new_path()
                    }
                } else {
                    generate_new_path()
                }
            } else {
                commands.entity(quakka_e).try_remove::<FollowPath>();
            }
        }
    }
}
