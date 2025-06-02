use bevy::{
    color::palettes::css::*,
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    prelude::*,
    render::texture::TRANSPARENT_IMAGE_HANDLE,
    ui::FocusPolicy,
};
use strum_macros::EnumIter;

use crate::global::*;

#[derive(Component, Default)]
pub struct SelectedCard;

#[derive(Component)]
pub struct DeckBarRoot;

#[derive(Component)]
struct HoverSprite;

#[derive(Component, Clone, Copy, Debug, EnumIter)]
pub enum Card {
    Empty,
    Farmer,
    Quakka,
    Waterball,
}

impl Card {
    pub fn is_empty(&self) -> bool {
        match self {
            Card::Empty => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Card::Empty => "Empty".to_string(),
            Card::Farmer => "Farmer".to_string(),
            Card::Quakka => "Quakka".to_string(),
            Card::Waterball => "Waterball".to_string(),
        }
    }
}

pub fn deckbar(app: &mut App) {
    app.add_systems(Startup, (initialize_deckbar, spawn_hover_sprite))
        .add_systems(
            FixedUpdate,
            (
                highlight_card_on_hover,
                select_card_on_click,
                hover_sprite_when_card_selected,
                update_card_image,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_observer(remove_selected_card_style)
        .add_observer(add_selected_card_style);
}

fn initialize_deckbar(mut commands: Commands) {
    commands
        .spawn((
            DeckBarRoot,
            Node {
                display: Display::None,
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
            fn get_empty_card_node_bundle() -> impl Bundle {
                return (
                    Node {
                        height: Val::Px(100.0),
                        width: Val::Px(80.0),
                        ..default()
                    },
                    BackgroundColor(MAROON.into()),
                    Button,
                    Card::Empty,
                );
            }

            parent.spawn(get_empty_card_node_bundle());
            parent.spawn(get_empty_card_node_bundle());
            parent.spawn(get_empty_card_node_bundle());
            parent.spawn(get_empty_card_node_bundle());
        });
}

pub fn show_deckbar(mut deck_bar: Single<&mut Node, With<DeckBarRoot>>) {
    deck_bar.display = Display::Flex;
}

fn remove_selected_card_style(
    trigger: Trigger<OnRemove, SelectedCard>,
    mut node_q: Query<&mut Node>,
) {
    let e = trigger.target();
    let mut card_style = node_q.get_mut(e).unwrap();
    card_style.right = Val::ZERO;
}

fn add_selected_card_style(trigger: Trigger<OnAdd, SelectedCard>, mut node_q: Query<&mut Node>) {
    let e = trigger.target();
    let mut card_style = node_q.get_mut(e).unwrap();
    card_style.right = Val::Px(30.0);
}

pub fn clear_deckbar(
    cards: Query<Entity, With<Card>>,
    selected_card: Option<Single<Entity, With<SelectedCard>>>,
    mut commands: Commands,
) {
    for card in cards {
        commands.entity(card).insert(Card::Empty);
    }

    if let Some(e) = selected_card {
        commands.entity(e.into_inner()).remove::<SelectedCard>();
    }
}

fn update_card_image(
    cards: Query<(Entity, &Card), Changed<Card>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (e, card) in cards {
        commands
            .entity(e)
            .insert(get_image_node(card, &asset_server));
    }

    fn get_image_node(card: &Card, asset_server: &Res<AssetServer>) -> ImageNode {
        let image = match card {
            Card::Farmer => asset_server.load("farmer_mugshot.png"),
            Card::Quakka => asset_server.load("quakka_mugshot.png"),
            Card::Waterball => asset_server.load("waterball_mugshot.png"),
            Card::Empty => TRANSPARENT_IMAGE_HANDLE,
        };

        ImageNode {
            image,
            color: GREY.into(),
            ..Default::default()
        }
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
            _ => GREY.into(),
        }
    }
}

fn spawn_hover_sprite(mut commands: Commands) {
    commands.spawn((
        HoverSprite,
        Transform::default(),
        Sprite {
            color: Color::NONE,
            ..default()
        },
    ));
}

fn hover_sprite_when_card_selected(
    mut commands: Commands,

    hover_sprite: Single<Entity, With<HoverSprite>>,

    selected_card: Option<Single<&Card, With<SelectedCard>>>,
    asset_server: Res<AssetServer>,
    cursor_world_coords: Res<CursorWorldCoords>,
) {
    let mut hide_hover_sprite = || {
        commands.entity(*hover_sprite).insert(Sprite {
            color: Color::NONE,
            ..default()
        });
    };

    if let Some(selected_card) = selected_card {
        match selected_card.into_inner() {
            Card::Farmer => {
                commands.entity(*hover_sprite).insert(Sprite {
                    image: asset_server.load("farmer.png"),
                    custom_size: Some(FARMER_SIZE),
                    color: Color::linear_rgba(1., 1., 1., 0.5),
                    ..default()
                });
            }
            Card::Quakka => {
                commands.entity(*hover_sprite).insert(Sprite {
                    image: asset_server.load("quakka.png"),
                    custom_size: Some(QUAKKA_SIZE),
                    color: Color::linear_rgba(1., 1., 1., 0.5),
                    ..default()
                });
            }
            Card::Waterball => {
                commands.entity(*hover_sprite).insert(Sprite {
                    image: asset_server.load("waterball.png"),
                    custom_size: Some(WATERBALL_SIZE),
                    color: Color::linear_rgba(1., 1., 1., 0.5),
                    ..default()
                });
            }
            Card::Empty => hide_hover_sprite(),
        }
    } else {
        hide_hover_sprite()
    }

    let cursor_world_coords = cursor_world_coords.0;
    commands
        .entity(*hover_sprite)
        .entry::<Transform>()
        .and_modify(move |mut transform| {
            transform.translation = cursor_world_coords.extend(0.);
        });
}

fn select_card_on_click(
    mut interaction_query: Query<
        (&Interaction, Entity),
        (Changed<Interaction>, With<Button>, With<Card>),
    >,
    old_selected_card: Option<Single<Entity, With<SelectedCard>>>,
    cards_q: Query<&Card>,

    mut commands: Commands,
) {
    let old_selected_card: Option<Entity> = {
        if let Some(old_selected_card) = old_selected_card {
            Some(old_selected_card.into_inner())
        } else {
            None
        }
    };

    for (interaction, card_clicked_e) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            return;
        }

        let card_clicked = cards_q.get(card_clicked_e).unwrap();

        if card_clicked.is_empty() {
            return;
        }

        if let Some(old_selected_card) = old_selected_card {
            commands.entity(old_selected_card).remove::<SelectedCard>();
        }

        commands.entity(card_clicked_e).insert(SelectedCard);
    }
}

#[derive(Default)]
pub struct DeleteSelectedCard;
impl Command for DeleteSelectedCard {
    fn apply(self, mut world: &mut World) {
        let selected_card: Entity = world
            .query_filtered::<Entity, With<SelectedCard>>()
            .single(&mut world)
            .unwrap();

        let mut selected_card = world.get_entity_mut(selected_card).unwrap();

        selected_card.remove::<SelectedCard>();
        selected_card.insert(Card::Empty);
    }
}

pub struct PushToDeckbar(pub Card);

impl Command for PushToDeckbar {
    fn apply(self, world: &mut World) -> () {
        let deck = world
            .query_filtered::<&Children, With<DeckBarRoot>>()
            .single(world)
            .unwrap();

        let empty_card = deck
            .iter()
            .find(|e| world.get::<Card>(*e).unwrap().is_empty());

        if let Some(empty_card) = empty_card {
            *world.get_mut::<Card>(empty_card).unwrap() = self.0;
        }
    }
}
