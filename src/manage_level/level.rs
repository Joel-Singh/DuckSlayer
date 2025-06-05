use bevy::prelude::*;
use serde::Serialize;

use crate::{
    card::Card,
    global::{NEST_POSITIONS, QUAKKA_STARTING_POSITION},
};

#[derive(Serialize, Default)]
pub struct Level {
    pub cards: Vec<(Card, Vec2)>,
    pub starting_deckbar: Vec<Card>,
}

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
