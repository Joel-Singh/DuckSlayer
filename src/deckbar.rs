use bevy::{color::palettes::css::*, prelude::*};

use crate::global::*;

#[derive(Resource, Default)]
struct SelectedCard(Option<Entity>);

#[derive(Component)]
struct DeckBarRoot;

#[derive(Component)]
struct Card {
    troop: Option<Troop>,
}

enum Troop {
    Farmer,
}

pub fn deckbar(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), initialize_deckbar)
        .add_systems(
            FixedUpdate,
            (highlight_card_on_hover, select_card_on_click).run_if(in_state(GameState::InGame)),
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

        let card_clicked = cards_q.get(entity).expect("Clicked entity is Card");

        if card_clicked.troop.is_none() {
            return;
        }

        if let Some(old_selected_card) = selected_card.0 {
            let mut old_selected_card = nodes
                .get_mut(old_selected_card)
                .expect("Selected Card Entity has Node");

            old_selected_card.right = Val::ZERO;
        }

        selected_card.0 = Some(entity);
        let mut selected_card_node = nodes.get_mut(entity).unwrap();
        selected_card_node.right = Val::Px(30.0);
    }
}
