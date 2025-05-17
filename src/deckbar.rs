use bevy::{color::palettes::css::*, prelude::*, ui::FocusPolicy};

use crate::global::*;

#[derive(Resource, Default)]
pub struct SelectedCard(pub Option<(Entity, Troop)>);

#[derive(Component)]
struct DeckBarRoot;

#[derive(Component)]
struct HoverSprite;

#[derive(Component)]
pub struct Card {
    pub troop: Option<Troop>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Troop {
    Farmer,
}

pub fn deckbar(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        (initialize_deckbar, spawn_hover_sprite),
    )
    .add_systems(
        FixedUpdate,
        (
            highlight_card_on_hover,
            select_card_on_click,
            hover_sprite_when_card_selected,
        )
            .run_if(in_state(GameState::InGame)),
    )
    .init_resource::<SelectedCard>();
}

fn initialize_deckbar(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            FocusPolicy::Block,
        ))
        .with_children(|parent| {
            let spawn_card_node = |parent: &mut ChildSpawnerCommands, troop: Option<Troop>| {
                let mut card_node = parent.spawn((
                    Node {
                        height: Val::Px(100.0),
                        width: Val::Px(80.0),
                        ..default()
                    },
                    BackgroundColor(MAROON.into()),
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

fn spawn_hover_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        HoverSprite,
        Transform::default(),
        Sprite::from_image(asset_server.load("farmer.png")),
    ));
}

fn hover_sprite_when_card_selected(
    mut commands: Commands,
    mut current_sprite: Local<Option<Troop>>,

    hover_sprite: Single<Entity, With<HoverSprite>>,

    selected_card: Res<SelectedCard>,
    asset_server: Res<AssetServer>,
    cursor_world_coords: Res<CursorWorldCoords>,
) {
    if let Some((_, troop)) = selected_card.0 {
        match troop {
            Troop::Farmer => {
                commands.entity(*hover_sprite).insert(Sprite {
                    image: asset_server.load("farmer.png"),
                    custom_size: Some(FARMER_SIZE),
                    color: Color::linear_rgba(1., 1., 1., 0.5),
                    ..default()
                });

                *current_sprite = Some(Troop::Farmer);
            }
        }
    } else {
        commands.entity(*hover_sprite).insert(Sprite::default());
    }

    let cursor_world_coords = cursor_world_coords.0;
    commands
        .entity(*hover_sprite)
        .entry::<Transform>()
        .and_modify(move |mut transform| {
            transform.translation = dbg!(cursor_world_coords.extend(0.));
        });
}

fn select_card_on_click(
    mut interaction_query: Query<
        (&Interaction, Entity),
        (Changed<Interaction>, With<Button>, With<Card>),
    >,
    mut selected_card: ResMut<SelectedCard>,
    mut nodes: Query<&mut Node>,
    cards_q: Query<&Card>,
) {
    for (interaction, entity) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            return;
        }

        let card_clicked = cards_q.get(entity).unwrap();

        if card_clicked.troop.is_none() {
            return;
        }

        if let Some(old_selected_card) = selected_card.0 {
            let mut old_selected_card = nodes
                .get_mut(old_selected_card.0)
                .expect("Selected Card Entity has Node");

            old_selected_card.right = Val::ZERO;
        }

        selected_card.0 = Some((entity, card_clicked.troop.unwrap()));
        let mut selected_card_node = nodes.get_mut(entity).unwrap();
        selected_card_node.right = Val::Px(30.0);
    }
}

#[derive(Default)]
pub struct DeleteSelectedCard;
impl Command for DeleteSelectedCard {
    fn apply(self, world: &mut World) {
        let mut selected_card_res = world.get_resource_mut::<SelectedCard>().unwrap();
        let (selected_card_e, _) = selected_card_res.0.unwrap();

        selected_card_res.0 = None;

        let mut selected_card = world.get_entity_mut(selected_card_e).unwrap();

        selected_card.insert((ImageNode::default(), Card { troop: None }));

        let mut selected_card_node = selected_card.get_mut::<Node>().unwrap();
        selected_card_node.right = Val::ZERO;
    }
}
