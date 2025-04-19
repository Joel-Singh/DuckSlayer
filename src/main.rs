use bevy::{color::palettes::css::*, prelude::*};

#[derive(Component)]
struct Quacka;

#[derive(Component)]
struct Nest;

#[derive(Component)]
struct DeckBarRoot;

const QUACKA_SPEED: f32 = 75.0;
const QUACKA_HIT_DISTANCE: f32 = 50.0;

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
        .add_systems(FixedUpdate, quacka_go_to_nest)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("quacka.png"),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 200., 0.),
            ..default()
        },
        Quacka,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("nest.png"),
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(
                0. - DECK_WIDTH - 0.15 * ARENA_WIDTH,
                0.0 - 0.25 * SCREEN_HEIGHT,
                0.,
            ),
            ..default()
        },
        Nest,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("nest.png"),
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(
                0. - DECK_WIDTH + 0.15 * ARENA_WIDTH,
                0.0 - 0.25 * SCREEN_HEIGHT,
                0.,
            ),
            ..default()
        },
        Nest,
    ));

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
}

fn quacka_go_to_nest(
    mut quackas: Query<&mut Transform, (With<Quacka>, Without<Nest>)>,
    nest: Query<&Transform, (With<Nest>, Without<Quacka>)>,
    time: Res<Time>,
) {
    for mut quacka in quackas.iter_mut() {
        let nest = nest
            .iter()
            .max_by(|a, b| {
                let a_distance = quacka.translation.distance(a.translation);
                let b_distance = quacka.translation.distance(b.translation);
                b_distance.partial_cmp(&a_distance).unwrap()
            })
            .unwrap();

        let mut difference = nest.translation - quacka.translation;
        difference = difference.normalize();

        if quacka.translation.distance(nest.translation) < QUACKA_HIT_DISTANCE {
            continue;
        } else {
            quacka.translation =
                quacka.translation + (difference) * time.delta_secs() * QUACKA_SPEED;
        }
    }
}
