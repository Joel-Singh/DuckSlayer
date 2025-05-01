use bevy::{color::palettes::css::*, input::common_conditions::*, prelude::*};

#[derive(Component)]
struct Quacka;

#[derive(Component)]
#[require(Chaseable)]
struct Farmer;

#[derive(Component)]
struct GoingToBridge;

#[derive(Component)]
struct Bridge;

#[derive(Component)]
#[require(Chaseable)]
struct Nest;

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

#[derive(Component)]
struct HealthBar;

#[derive(Component, Default)]
struct Chaseable;

#[derive(Component)]
struct DeckBarRoot;

const QUACKA_SPEED: f32 = 25.0;
const QUACKA_HIT_DISTANCE: f32 = 50.0;

const FARMER_SPEED: f32 = 25.0;

const SCREEN_WIDTH: f32 = 1366.0;
const SCREEN_HEIGHT: f32 = 768.0;

const ARENA_WIDTH: f32 = 0.8 * SCREEN_WIDTH;
const DECK_WIDTH: f32 = 0.1 * SCREEN_WIDTH;

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
                quacka_chase,
                farmer_go_to_bridge,
                farmer_go_up,
                update_healthbars,
                spawn_farmer.run_if(input_pressed(MouseButton::Left)),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_farmer(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
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
        GoingToBridge,
    ));
}

fn quacka_chase(
    mut quackas: Query<&mut Transform, (With<Quacka>, Without<Nest>)>,
    chaseables: Query<&Transform, (With<Chaseable>, Without<Quacka>)>,
    time: Res<Time>,
) {
    for mut quacka in quackas.iter_mut() {
        let closest_chaseable = chaseables
            .iter()
            .max_by(|a, b| {
                let a_distance = quacka.translation.distance(a.translation);
                let b_distance = quacka.translation.distance(b.translation);
                b_distance.partial_cmp(&a_distance).unwrap()
            })
            .unwrap();

        let mut difference = closest_chaseable.translation - quacka.translation;
        difference = difference.normalize();

        if quacka.translation.distance(closest_chaseable.translation) < QUACKA_HIT_DISTANCE {
            continue;
        } else {
            quacka.translation += (difference) * time.delta_secs() * QUACKA_SPEED;
        }
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

fn update_healthbars(
    mut commands: Commands,
    mut healthbar_q: Query<(Entity, &Parent), With<HealthBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    health: Query<&Health>,
) {
    for (healthbar, troop) in healthbar_q.iter_mut() {
        let health = health.get(troop.get()).unwrap();
        let health_percentage = health.current / health.max;

        commands.entity(healthbar).insert(Mesh2d(
            meshes.add(Rectangle::new(health_percentage * 100.0, 10.0)),
        ));

        commands.entity(healthbar).insert_if_new(
            MeshMaterial2d(materials.add(Color::from(RED)))
        );
    }
}

fn spawn_entities(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let quakka = commands
        .spawn((
            Sprite {
                image: asset_server.load("quacka.png"),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(0., 200., 0.),
                ..default()
            },
            Health {
                current: 100.0,
                max: 100.0,
            },
            Quacka,
        ))
        .id();

    let quakka_healthbar = commands
        .spawn((
            Transform::from_xyz(0., 60., 0.),
            HealthBar,
        ))
        .id();

    commands.entity(quakka).add_children(&[quakka_healthbar]);

    spawn_nest(
        Vec3::new(
                0. - DECK_WIDTH - 0.15 * ARENA_WIDTH,
                0.0 - 0.25 * SCREEN_HEIGHT,
                0.
        ),
        &mut commands,
        &asset_server
    );

    spawn_nest(
        Vec3::new(
                0. - DECK_WIDTH + 0.15 * ARENA_WIDTH,
                0.0 - 0.25 * SCREEN_HEIGHT,
                0.,
        ),
        &mut commands,
        &asset_server
    );

    commands
        .spawn((
            DeckBarRoot,
            Node {
                display: Display::Flex,
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                width: Val::Px(DECK_WIDTH * 0.8),
                height: Val::Vh(100.),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(5.)),
                margin: UiRect::left(Val::Auto),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            BorderColor(RED.into()),
        ))
        .with_children(|parent| {
            let card_node: Node = Node {
                height: Val::Px(100.0),
                width: Val::Px(80.0),
                ..default()
            };

            parent.spawn((
                Node {
                    ..card_node.clone()
                },
                BackgroundColor(MAROON.into()),
            ));

            parent.spawn((
                Node {
                    ..card_node.clone()
                },
                BackgroundColor(MAROON.into()),
            ));

            parent.spawn((
                Node {
                    ..card_node.clone()
                },
                BackgroundColor(MAROON.into()),
            ));

            parent.spawn((
                Node {
                    ..card_node.clone()
                },
                BackgroundColor(MAROON.into()),
            ));
        });

    commands.spawn((
        Bridge,
        Transform {
            translation: Vec3::new(
                0. - DECK_WIDTH - 0.15 * ARENA_WIDTH,
                0.0 + 0.25 * SCREEN_HEIGHT,
                0.,
            ),
            ..default()
        },
    ));
}

fn spawn_nest(
    translation: Vec3,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
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
        Nest,
    ));
}
