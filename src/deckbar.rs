use crate::{
    card::{Card, CardConsts, MaybeCard},
    ingame_ui_root::InGameUiRoot,
    volume_settings::VolumeSettings,
};

use bevy::{
    audio::PlaybackMode,
    color::palettes::css::*,
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    prelude::*,
    render::texture::TRANSPARENT_IMAGE_HANDLE,
};

use crate::global::*;

#[derive(Component, Default)]
pub struct SelectedCard;

#[derive(Component)]
pub struct DeckBarRoot;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitializeDeckbar;

#[derive(Component)]
struct HoverSprite;

pub fn deckbar(app: &mut App) {
    app.add_systems(
        Startup,
        (
            (initialize_deckbar, spawn_hover_sprite),
            remove_card_on_right_click_in_editor,
        )
            .chain(),
    )
    .add_systems(
        FixedUpdate,
        (
            highlight_card,
            play_sound_on_select,
            select_card_on_click,
            hover_sprite_when_card_selected,
            update_card_image,
        )
            .run_if(in_state(GameState::InGame)),
    )
    .add_systems(
        OnExit(GameState::InGame),
        (hide_hover_sprite, deselect_card),
    )
    .add_observer(remove_selected_card_style)
    .add_observer(add_selected_card_style);
}

fn initialize_deckbar(mut commands: Commands, ingame_ui_root: Res<InGameUiRoot>) {
    commands.entity(**ingame_ui_root).with_children(|p| {
        p.spawn((
            DeckBarRoot,
            Node {
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                width: Val::Px(DECK_WIDTH * 0.8),
                height: Val::Vh(100.),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(5.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            BorderColor(RED.into()),
            BackgroundColor(GREY.into()),
            Button, // So that it is taken into account for is_pointer_over_ui
        ))
        .with_children(|p| {
            fn get_empty_card_node_bundle() -> impl Bundle {
                return (
                    Node {
                        height: Val::Px(100.0),
                        width: Val::Px(80.0),
                        ..default()
                    },
                    BackgroundColor(MAROON.into()),
                    Button,
                    MaybeCard(None),
                );
            }

            p.spawn(get_empty_card_node_bundle());
            p.spawn(get_empty_card_node_bundle());
            p.spawn(get_empty_card_node_bundle());
            p.spawn(get_empty_card_node_bundle());
        });
    });
}

fn remove_selected_card_style(
    trigger: Trigger<OnRemove, SelectedCard>,
    mut node_q: Query<&mut Node>,
    mut image_node_q: Query<&mut ImageNode>,
) {
    let e = trigger.target();

    let mut node = node_q.get_mut(e).unwrap();
    node.right = Val::ZERO;

    let mut image_node = image_node_q.get_mut(e).unwrap();
    image_node.color = GREY.into();
}

fn add_selected_card_style(
    trigger: Trigger<OnAdd, SelectedCard>,
    mut node_q: Query<&mut Node>,
    mut image_node_q: Query<&mut ImageNode>,
) {
    let e = trigger.target();

    let mut node = node_q.get_mut(e).unwrap();
    node.right = Val::Px(30.0);

    let mut image_node = image_node_q.get_mut(e).unwrap();
    image_node.color = Color::WHITE;
}

pub fn clear_deckbar(
    cards: Query<Entity, With<MaybeCard>>,
    selected_card: Option<Single<Entity, With<SelectedCard>>>,
    mut commands: Commands,
) {
    for card in cards {
        commands.entity(card).insert(MaybeCard(None));
    }

    if let Some(e) = selected_card {
        commands.entity(e.into_inner()).remove::<SelectedCard>();
    }
}

fn update_card_image(
    cards: Query<(Entity, &MaybeCard), Changed<MaybeCard>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (e, card) in cards {
        commands
            .entity(e)
            .insert(get_image_node(card, &asset_server));
    }

    fn get_image_node(card: &MaybeCard, asset_server: &Res<AssetServer>) -> ImageNode {
        let image = match card.0 {
            None => TRANSPARENT_IMAGE_HANDLE,
            Some(card) => match card {
                Card::Farmer => asset_server.load("farmer-mugshot.png"),
                Card::Quakka => asset_server.load("quakka-mugshot.png"),
                Card::Waterball => asset_server.load("waterball-mugshot.png"),
                Card::Nest => asset_server.load("nest-mugshot.png"),
            },
        };

        ImageNode {
            image,
            color: GREY.into(),
            ..Default::default()
        }
    }
}

fn highlight_card(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, Has<SelectedCard>),
        (Changed<Interaction>, With<Button>, With<MaybeCard>),
    >,
) {
    for (interaction, mut image_node, is_selected) in &mut interaction_query {
        if *interaction == Interaction::Hovered {
            image_node.color = Color::WHITE;
        } else if !is_selected {
            image_node.color = GREY.into();
        }
    }
}

fn play_sound_on_select(
    mut interaction_query: Query<(&Interaction, &MaybeCard), (Changed<Interaction>, With<Button>)>,
    volume_settings: Res<VolumeSettings>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, card) in &mut interaction_query {
        if *interaction == Interaction::Hovered && card.is_some() {
            commands.spawn((
                AudioPlayer::new(asset_server.load("card-select.ogg")),
                PlaybackSettings {
                    volume: volume_settings.get_sfx(),
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
            ));
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

fn hide_hover_sprite(mut hover_sprite: Single<&mut Sprite, With<HoverSprite>>) {
    hover_sprite.color = Color::NONE;
}

fn hover_sprite_when_card_selected(
    mut commands: Commands,

    hover_sprite: Single<Entity, With<HoverSprite>>,

    selected_card: Option<Single<&MaybeCard, With<SelectedCard>>>,
    asset_server: Res<AssetServer>,
    cursor_world_coords: Res<CursorWorldCoords>,

    card_consts: Res<CardConsts>,
) {
    let mut hide_hover_sprite = || {
        commands.entity(*hover_sprite).insert(Sprite {
            color: Color::NONE,
            ..default()
        });
    };

    if let Some(selected_card) = selected_card {
        match selected_card.0 {
            None => {
                hide_hover_sprite();
            }
            Some(selected_card) => {
                commands.entity(*hover_sprite).insert(Sprite {
                    color: Color::linear_rgba(1., 1., 1., 0.5),
                    ..selected_card.get_sprite(&asset_server, &card_consts)
                });
            }
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
        (Changed<Interaction>, With<Button>, With<MaybeCard>),
    >,
    old_selected_card: Option<Single<Entity, With<SelectedCard>>>,
    cards_q: Query<&MaybeCard>,

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

        match card_clicked.0 {
            None => {}
            Some(_) => {
                if let Some(old_selected_card) = old_selected_card {
                    commands.entity(old_selected_card).remove::<SelectedCard>();
                }

                commands.entity(card_clicked_e).insert(SelectedCard);
            }
        }
    }
}

fn remove_card_on_right_click_in_editor(
    cards_q: Query<Entity, With<MaybeCard>>,
    mut commands: Commands,
) {
    for card in cards_q {
        commands.entity(card).insert(Pickable::default());
        commands.entity(card).observe(
            |trigger: Trigger<Pointer<Click>>,
             in_editor: Res<InEditorRes>,
             mut commands: Commands| {
                let right_click = trigger.button == PointerButton::Secondary;
                if right_click && **in_editor {
                    commands.entity(trigger.target).insert(MaybeCard(None));
                    commands.entity(trigger.target).remove::<SelectedCard>();
                }
            },
        );
    }
}

pub fn deselect_card(
    selected_card: Option<Single<Entity, With<SelectedCard>>>,
    mut commands: Commands,
) {
    if selected_card.is_some() {
        commands
            .entity(*selected_card.unwrap())
            .remove::<SelectedCard>();
    }
}

pub fn select_card(to_select: usize) -> ScheduleConfigs<ScheduleSystem> {
    if to_select > 3 {
        panic!("Invalid Card deck index");
    }

    (move |old_selected_card: Option<Single<Entity, With<SelectedCard>>>,
           deck: Single<&Children, With<DeckBarRoot>>,
           mut commands: Commands,
           cards: Query<&MaybeCard>| {
        if let Some(old_selected_card) = old_selected_card {
            commands
                .entity(old_selected_card.into_inner())
                .remove::<SelectedCard>();
        }

        let mut card: Entity = Entity::PLACEHOLDER;
        for (i, card_e) in deck.iter().enumerate() {
            if i == to_select {
                card = card_e;
                break;
            }
        }

        if cards.get(card).unwrap().0.is_some() {
            commands.entity(card).insert(SelectedCard);
        }
    })
    .into_configs()
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
        selected_card.insert(MaybeCard(None));
    }
}

pub struct PushToDeckbar(pub Card);

impl Command for PushToDeckbar {
    fn apply(self, world: &mut World) -> () {
        let new_card = self.0;

        let deck = world
            .query_filtered::<&Children, With<DeckBarRoot>>()
            .single(world)
            .unwrap();

        let empty_card = deck
            .iter()
            .find(|e| world.get::<MaybeCard>(*e).unwrap().0.is_none());

        if let Some(empty_card) = empty_card {
            *world.get_mut::<MaybeCard>(empty_card).unwrap() = MaybeCard(Some(new_card));
        }
    }
}
