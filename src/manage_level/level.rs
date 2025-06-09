
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, Farmer, Nest, Quakka},
    deckbar::DeckBarRoot,
};

#[derive(Serialize, Deserialize, Default, Asset, TypePath)]
pub struct Level {
    pub cards: Vec<(Card, Vec2)>,
    pub starting_deckbar: Vec<Card>,
}

impl Level {
    pub fn get_current(world: &mut World) -> Level {
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
}

pub fn level_plugin(app: &mut App) {
    app.add_plugins(JsonAssetPlugin::<Level>::new(&[]));
}
