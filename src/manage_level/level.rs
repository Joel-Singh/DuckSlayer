use bevy::prelude::*;
use serde::Serialize;

use crate::{
    card::Card,
    global::{NEST_POSITIONS, QUAKKA_STARTING_POSITION},
};

#[derive(Serialize)]
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

    pub fn print_to_out(&self) {
        println!("{}", serde_json::to_string_pretty(&self).unwrap());
    }
}
