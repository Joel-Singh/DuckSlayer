use bevy::{color::palettes::css::*, prelude::*, input::common_conditions::*, time::common_conditions::*};

use rand::Rng;

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

#[derive(Component)]
struct MainMenuRoot;


#[derive(Resource, Default)]
struct SecondsSurvived(i32);

#[derive(Component)]
struct SecondsSurvivedCounter;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum GameState {
    #[default]
    StartScreen,
    Paused,
    Unpaused
}

const QUAKKA_SPEED: f32 = 40.0;
const QUAKKA_HIT_DISTANCE: f32 = 50.0;
const QUAKKA_DAMAGE: f32 = 60.0;

const FARMER_SPEED: f32 = 100.0;

const SCREEN_WIDTH: f32 = 1366.0;
const SCREEN_HEIGHT: f32 = 768.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            //level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_start_screen, spawn_background_image))
        .add_systems(OnExit(GameState::StartScreen),
            (
                |mut commands: Commands, main_menu: Single<Entity, With<MainMenuRoot>>| {
                    let main_menu = main_menu.into_inner();
                    commands.entity(main_menu).despawn_recursive();
                },
                create_seconds_survived,
                restart
            )
        )
        .add_systems(
            FixedUpdate,
            (
                start_game_on_click.run_if(in_state(GameState::StartScreen)),
                (
                    quakka_chase_and_attack,
                    delete_dead_entities,
                    update_healthbars,
                    pause_when_dead_farmer
                )
                    .chain()
                    .run_if(in_state(GameState::Unpaused)),
                (
                    move_farmer_with_wasd,
                    update_counter
                ).run_if(in_state(GameState::Unpaused)),
                randomly_spawn_quakkas.run_if(
                    on_timer(Duration::from_secs_f32(1.)).and(in_state(GameState::Unpaused))
                ),
                increment_counter.run_if(on_timer(Duration::from_secs(1)).and(in_state(GameState::Unpaused))),
                tick_attacker_cooldowns,
                restart.run_if(in_state(GameState::Paused).and(input_just_pressed(KeyCode::Space))),
            ),
        )
        .init_state::<GameState>()
        .init_resource::<SecondsSurvived>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_background_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        Transform {
            translation: Vec3::new(0., 0., -1.),
            ..default()
        },
    ));
}

fn tick_attacker_cooldowns(mut attackers: Query<&mut Attacker>, time: Res<Time>) {
    for mut attacker in attackers.iter_mut() {
        if attacker.cooldown.mode() == TimerMode::Repeating {
            panic!("Attack cooldown should be once");
        }
        attacker.cooldown.tick(time.delta());
    }
}

fn setup_start_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        MainMenuRoot
    )).with_children(|p| {
            p.spawn((
                Text::new("DuckSlayer, click to start"),
                Node {
                    margin: UiRect::horizontal(Val::Auto),
                    ..default()
                },
            ));
        });
}

fn start_game_on_click(
    interactions: Query<&Interaction, Changed<Interaction>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    for interaction in interactions.iter() {
        if let Interaction::Pressed = interaction {
            game_state.set(GameState::Unpaused);
        }
    }
}

fn move_farmer_with_wasd(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut farmer_transform_q: Query<&mut Transform, With<Farmer>>
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

    if let Ok(mut transform) = farmer_transform_q.get_single_mut() {
        transform.translation += movement.normalize_or_zero() * time.delta_secs() * FARMER_SPEED;
    }
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

fn randomly_spawn_quakkas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    farmer: Query<(), With<Farmer>>
) {
    if let Ok(()) = farmer.get_single() {
        let mut rng = rand::rng();

        let x = rng.random_range(-SCREEN_WIDTH / 2.0..SCREEN_WIDTH / 2.0);
        let y = rng.random_range(-SCREEN_HEIGHT / 2.0..SCREEN_WIDTH / 2.0);

        commands
            .spawn((
                Sprite {
                    image: asset_server.load("quakka.png"),
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(x, y, 0.),
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
            ));


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

fn pause_when_dead_farmer(
    farmer: Query<&Farmer>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if let Err(_) = farmer.get_single() {
        game_state.set(GameState::Paused);
    }
}

fn create_seconds_survived(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        Text::new("0"),
        SecondsSurvivedCounter,
    ));
}

fn update_counter(
    seconds_survived: Res<SecondsSurvived>,
    mut counter: Single<&mut Text, With<SecondsSurvivedCounter>>,
) {
    counter.0 = seconds_survived.0.to_string();
}

fn increment_counter(
    mut seconds_survived: ResMut<SecondsSurvived>
) {
    seconds_survived.0 += 1;
}

fn restart(
    asset_server: Res<AssetServer>,
    quakkas: Query<Entity, With<Quakka>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut seconds_survived: ResMut<SecondsSurvived>,
    mut commands: Commands
) {
    seconds_survived.0 = 0;
    game_state.set(GameState::Unpaused);

    for quakka in quakkas.iter() {
        commands.entity(quakka).despawn_recursive();
    }


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
