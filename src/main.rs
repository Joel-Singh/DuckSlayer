use bevy::{color::palettes::css::*, input::common_conditions::*, prelude::*};

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
struct GoingToBridge;

enum Troop {
    Farmer
}

#[derive(Component)]
struct Card {
    troop: Option<Troop>
}

#[derive(Component)]
struct Bridge;

#[derive(Resource, Default)]
struct SelectedCard(Option<Entity>);

#[derive(Component)]
#[require(Chaseable)]
struct Nest;

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
struct DeckBarRoot;

const QUAKKA_SPEED: f32 = 75.0;
const QUAKKA_HIT_DISTANCE: f32 = 50.0;
const QUAKKA_DAMAGE: f32 = 60.0;

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
                (
                    quakka_chase_and_attack,
                    delete_dead_entities
                ).chain(),
                farmer_go_to_bridge,
                farmer_go_up,
                update_healthbars,
                spawn_farmer.run_if(input_pressed(MouseButton::Left)),
                tick_attacker_cooldowns,
                highlight_card_on_hover,
                select_card_on_click
            ),
        )
        .init_resource::<SelectedCard>()
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

fn tick_attacker_cooldowns(mut attackers: Query<&mut Attacker>, time: Res<Time>) {
    for mut attacker in attackers.iter_mut() {
        if attacker.cooldown.mode() == TimerMode::Repeating {
            panic!("Attack coolodwn should be once");
        }
        attacker.cooldown.tick(time.delta());
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

fn quakka_chase_and_attack(
    mut quakkas: Query<(&mut Transform, &mut Attacker), (With<Quakka>, Without<Nest>)>,
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


fn highlight_card_on_hover(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image_node) in &mut interaction_query {
        image_node.color = match *interaction {
            Interaction::Hovered => Color::WHITE,
            _ => GREY.into()
        }
    }
}

fn select_card_on_click(
    mut interaction_query: Query<
        (&Interaction, Entity),
        (Changed<Interaction>, (With<Button>, With<Card>)),
    >,
    mut selected_card: ResMut::<SelectedCard>,
    mut nodes: Query<&mut Node>
) {
    if let Some(old_selected_card) = selected_card.0 {
        let mut old_selected_card = nodes.get_mut(old_selected_card).expect("Selected Card Entity has Node");

        old_selected_card.right = Val::ZERO;
    }

    for (interaction, entity) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                selected_card.0 = Some(entity);

                let mut selected_card_node = nodes.get_mut(entity).unwrap();
                selected_card_node.right = Val::Px(30.0);
            },
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
                    Card {
                        troop: None
                    },
                    Button
                ));

                let image_node = match troop {
                    None => ImageNode::default(),
                    Some(ref troop) => match troop {
                        Troop::Farmer => ImageNode::new(asset_server.load("farmer_mugshot.png"))
                    }
                };

                card_node.insert((
                    image_node,
                    Card {
                        troop
                    },
                ));
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
