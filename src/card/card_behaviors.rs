use crate::global::get_left_river_rect;
use crate::global::get_middle_river_rect;
use crate::global::get_right_river_rect;
use crate::global::IsPointerOverUi;
use crate::global::HEALTHBAR_SIZE;
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
use bevy::prelude::*;
use debug::debug;
use farmer::farmer_plugin;
use farmer::kill_farmer_reaching_exit;
pub use follow_path::follow_paths;
use nest::nest_plugin;
use nest::nest_shoot;

#[derive(Component)]
#[require(LevelEntity, NestTarget, WaterballTarget)]
#[require(SpawnedCard(Card::Quakka))]
pub struct Quakka;

#[derive(Component)]
#[require(QuakkaTarget, LevelEntity, WalkAnim)]
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
                kill_farmer_reaching_exit,
                tick_attacker_cooldowns,
                quakka_chase_and_attack,
                explode_waterballs,
                nest_shoot,
                follow_paths,
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
    .add_plugins(walk_animation_plugin)
    .add_plugins(debug);
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

fn quakka_chase_and_attack(
    mut quakkas: Query<(&mut Transform, &mut Attacker, Entity), With<Quakka>>,
    mut quakka_targets: Query<
        (&Transform, Entity, &mut Health),
        (With<QuakkaTarget>, Without<Quakka>),
    >,
    time: Res<Time>,
    card_consts: Res<CardConsts>,
    mut commands: Commands,
) {
    for mut quakka in quakkas.iter_mut() {
        let quakka_e: Entity = quakka.2;

        let closest_chaseable = quakka_targets.iter_mut().min_by(|a, b| {
            let a_distance = quakka.0.translation.distance(a.0.translation);
            let b_distance = quakka.0.translation.distance(b.0.translation);
            a_distance.partial_cmp(&b_distance).unwrap()
        });

        if closest_chaseable.is_none() {
            commands
                .entity(quakka_e)
                .insert(walk_animation::CancelWalkAnim);
            continue;
        }

        let mut closest_chaseable = closest_chaseable.unwrap();

        let distance_to_chaseable = quakka
            .0
            .translation
            .distance(closest_chaseable.0.translation);

        let in_attack_distance = distance_to_chaseable < card_consts.quakka.range;
        if in_attack_distance {
            commands
                .entity(quakka_e)
                .insert(walk_animation::CancelWalkAnim);
        }

        if in_attack_distance && quakka.1.cooldown.finished() {
            quakka.1.cooldown.reset();
            closest_chaseable.2.current_health -= card_consts.quakka.damage;
        } else if !in_attack_distance {
            let mut to_chaseable = closest_chaseable.0.translation - quakka.0.translation;
            to_chaseable = to_chaseable.normalize();

            quakka.0.translation += to_chaseable * time.delta_secs() * card_consts.quakka.speed;
            commands
                .entity(quakka_e)
                .try_insert_if_new(walk_animation::WalkAnim::default());
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

mod follow_path {
    use bevy::{
        ecs::{component::HookContext, world::DeferredWorld},
        prelude::*,
    };
    use pathfinding::prelude::astar;

    use crate::global::{
        get_entire_map_rect, get_left_river_rect, get_middle_river_rect, get_right_river_rect,
    };

    #[derive(Component)]
    #[require(Transform)]
    #[component(on_add = generate_path)]
    pub struct FollowPath {
        goal: (i32, i32),
        path: Vec<Vec2>,
        current: usize,
        speed: f32,
    }

    impl FollowPath {
        pub fn new(goal: (i32, i32), speed: f32) -> Self {
            FollowPath {
                goal,
                speed,
                path: Vec::default(),
                current: usize::default(),
            }
        }
    }

    pub fn follow_paths(path_followers: Query<(&mut Transform, &mut FollowPath)>, time: Res<Time>) {
        for (mut transform, mut follow_path) in path_followers {
            const TOLERANCE: f32 = 1.0;
            let stop = follow_path.path[follow_path.current];

            let mut to = stop - transform.translation.truncate();
            to = to.normalize_or_zero();

            transform.translation += (to * follow_path.speed * time.delta_secs()).extend(0.0);
            if stop.distance(transform.translation.truncate()) < TOLERANCE
                && follow_path.current < follow_path.path.len() - 1
            {
                follow_path.current += 1;
            }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Pos(pub i32, pub i32);

    const ASTAR_RESOLUTION: i32 = 30;
    impl Pos {
        fn distance(&self, other: &Pos) -> u32 {
            let a = self.0 - other.0;
            let b = self.1 - other.1;
            ((a * a + b * b) as f32).sqrt() as u32
        }

        fn successors(&self) -> Vec<(Pos, u32)> {
            let &Pos(x, y) = self;
            vec![
                Pos(x + ASTAR_RESOLUTION, y),
                Pos(x - ASTAR_RESOLUTION, y),
                Pos(x, y + ASTAR_RESOLUTION),
                Pos(x, y - ASTAR_RESOLUTION),
            ]
            .into_iter()
            .filter(reachable)
            .map(|p| (p, 1))
            .collect()
        }
    }

    impl Into<Vec2> for Pos {
        fn into(self) -> Vec2 {
            Vec2::new(self.0 as f32, self.1 as f32)
        }
    }

    impl From<(i32, i32)> for Pos {
        fn from(item: (i32, i32)) -> Self {
            Pos(item.0, item.1)
        }
    }

    fn generate_path(mut world: DeferredWorld, context: HookContext) {
        let start = world
            .get::<Transform>(context.entity)
            .unwrap()
            .translation
            .clone();

        let mut follow_path = world.get_mut::<FollowPath>(context.entity).unwrap();

        let goal = follow_path.goal;

        let found_path = astar(
            &Pos(start.x as i32, start.y as i32),
            |p| p.successors(),
            |p| p.distance(&goal.into()) / 3,
            |p| p.distance(&goal.into()) <= ASTAR_RESOLUTION.try_into().unwrap(),
        );

        let Some((found_path, _)) = found_path else {
            panic!("Tried to generate an impossible path to {:?}", goal);
        };

        debug_assert!(follow_path.path.is_empty());
        for pos in found_path {
            follow_path.path.push(pos.into());
        }
    }

    fn reachable(pos: &Pos) -> bool {
        let pos: Vec2 = Vec2::new(pos.0 as f32, pos.1 as f32);

        get_entire_map_rect().contains(pos)
            && !get_left_river_rect().contains(pos)
            && !get_middle_river_rect().contains(pos)
            && !get_right_river_rect().contains(pos)
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
        if ev.state != ButtonState::Pressed || **is_pointer_over_ui || !in_bounds(**mouse_coords) {
            continue;
        }

        commands.queue(SpawnCard::new(selected_card, mouse_coords.0));
        commands.queue(DeleteSelectedCard::default());
    }

    fn in_bounds(v: Vec2) -> bool {
        !get_left_river_rect().contains(v)
            && !get_middle_river_rect().contains(v)
            && !get_right_river_rect().contains(v)
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
use walk_animation::walk_animation_plugin;
use walk_animation::WalkAnim;

use super::Card;
use super::CardConsts;
use super::MaybeCard;
use super::SpawnCard;

mod debug {
    use crate::{card::CardConsts, debug::in_debug};

    use super::Nest;
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
            (show_nest_attack_radius)
                .run_if(in_debug.and(resource_equals(IsSpawnedCardDebugOverlayEnabled(true)))),
        )
        .init_resource::<IsSpawnedCardDebugOverlayEnabled>();
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
