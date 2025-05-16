use bevy::{color::palettes::css::*, input::common_conditions::*, prelude::*};

use std::time::Duration;

mod global;
use global::*;

mod titlescreen;
use titlescreen::*;

mod troops;
use troops::*;

enum Troop {
    Farmer,
}

#[derive(Component)]
struct Card {
    troop: Option<Troop>,
}

#[derive(Resource, Default)]
struct SelectedCard(Option<Entity>);

#[derive(Component)]
struct DeckBarRoot;

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
        .add_plugins(title_screen)
        .add_plugins(troops)
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::InGame), spawn_entities)
        .add_systems(
            FixedUpdate,
            (highlight_card_on_hover, select_card_on_click).run_if(in_state(GameState::InGame)),
        )
        .init_state::<GameState>()
        .init_resource::<SelectedCard>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn highlight_card_on_hover(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image_node) in &mut interaction_query {
        image_node.color = match *interaction {
            Interaction::Hovered => Color::WHITE,
            _ => GREY.into(),
        }
    }
}

fn select_card_on_click(
    mut interaction_query: Query<
        (&Interaction, Entity),
        (Changed<Interaction>, (With<Button>, With<Card>)),
    >,
    mut selected_card: ResMut<SelectedCard>,
    mut nodes: Query<&mut Node>,
) {
    if let Some(old_selected_card) = selected_card.0 {
        let mut old_selected_card = nodes
            .get_mut(old_selected_card)
            .expect("Selected Card Entity has Node");

        old_selected_card.right = Val::ZERO;
    }

    for (interaction, entity) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                selected_card.0 = Some(entity);

                let mut selected_card_node = nodes.get_mut(entity).unwrap();
                selected_card_node.right = Val::Px(30.0);
            }
            _ => {}
        };
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

    spawn_nest(
        Vec3::new(
            0. - DECK_WIDTH - 0.15 * ARENA_WIDTH,
            0.0 - 0.25 * SCREEN_HEIGHT,
            0.,
        ),
        &mut commands,
        &asset_server,
    );

    spawn_nest(
        Vec3::new(
            0. - DECK_WIDTH + 0.15 * ARENA_WIDTH,
            0.0 - 0.25 * SCREEN_HEIGHT,
            0.,
        ),
        &mut commands,
        &asset_server,
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
            let spawn_card_node = |parent: &mut ChildBuilder, troop: Option<Troop>| {
                let mut card_node = parent.spawn((
                    Node {
                        height: Val::Px(100.0),
                        width: Val::Px(80.0),
                        ..default()
                    },
                    BackgroundColor(MAROON.into()),
                    Card { troop: None },
                    Button,
                ));

                let image_node = match troop {
                    None => ImageNode::default(),
                    Some(ref troop) => match troop {
                        Troop::Farmer => ImageNode::new(asset_server.load("farmer_mugshot.png")),
                    },
                };

                card_node.insert((image_node, Card { troop }));
            };

            spawn_card_node(parent, Some(Troop::Farmer));
            spawn_card_node(parent, None);
            spawn_card_node(parent, None);
            spawn_card_node(parent, None);
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

fn add_healthbar_child(e: Entity, height: f32, commands: &mut Commands) {
    let healthbar = commands
        .spawn((Transform::from_xyz(0., height, 1.), HealthBar))
        .id();

    commands.entity(e).add_children(&[healthbar]);
}

fn spawn_nest(translation: Vec3, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let nest = commands
        .spawn((
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
            },
            Nest,
        ))
        .id();

    add_healthbar_child(nest, 60., commands);
}
