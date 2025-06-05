use bevy::prelude::*;

use crate::{
    card::Card,
    global::{NEST_POSITIONS, QUAKKA_STARTING_POSITION},
};

pub struct Level {
    pub cards: Vec<(Card, Vec2)>,
    pub nest_locations: Vec<Vec2>,
    pub starting_deckbar: Vec<Card>,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            cards: vec![
                (Card::Quakka, QUAKKA_STARTING_POSITION),
                (Card::Nest, NEST_POSITIONS.0.into()),
                (Card::Nest, NEST_POSITIONS.1.into()),
            ],
            nest_locations: vec![],
            starting_deckbar: vec![Card::Farmer],
        }
    }
}

impl Level {
    pub fn clear(&mut self) {
        *self = Level {
            cards: Vec::new(),
            nest_locations: Vec::new(),
            starting_deckbar: Vec::new(),
        };
    }
}
