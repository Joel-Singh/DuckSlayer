use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, Farmer, Nest, Quakka},
    deckbar::DeckBarRoot,
    global::{NEST_POSITIONS, QUAKKA_STARTING_POSITION},
};

#[derive(Serialize, Deserialize, Default)]
pub struct Level {
    pub cards: Vec<(Card, Vec2)>,
    pub starting_deckbar: Vec<Card>,
}

impl Level {
    pub fn get_first_level() -> Level {
        Level {
            cards: vec![
                (Card::Quakka, QUAKKA_STARTING_POSITION),
                (Card::Nest, NEST_POSITIONS.0.into()),
                (Card::Nest, NEST_POSITIONS.1.into()),
            ],
            starting_deckbar: vec![Card::Farmer],
        }
    }
}

pub fn get_current_level(world: &mut World) -> Level {
    let mut level = Level::default();

    let mut cards = world.query::<(&Transform, Has<Quakka>, Has<Farmer>, Has<Nest>)>();
    for (transform, is_quakka, is_farmer, is_nest) in cards.iter(world) {
        if is_quakka {
            level
                .cards
                .push((Card::Quakka, transform.translation.truncate()));
        } else if is_farmer {
            level
                .cards
                .push((Card::Farmer, transform.translation.truncate()));
        } else if is_nest {
            level
                .cards
                .push((Card::Nest, transform.translation.truncate()));
        }
    }

    let deck = world
        .query_filtered::<&Children, With<DeckBarRoot>>()
        .single(world)
        .unwrap()
        .iter()
        .map(|e| world.get::<Card>(e).unwrap());

    for card in deck {
        level.starting_deckbar.push(*card);
    }

    level
}
