use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, Farmer, MaybeCard, Nest, Quakka},
    deckbar::DeckBarRoot,
};

#[derive(Serialize, Deserialize)]
pub struct DeathGoal {
    pub card: Card,
    pub count_dead: u32,
}

#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct Level {
    pub cards: Vec<(Card, Vec2)>,
    pub starting_deckbar: Vec<Card>,
    pub win_condition: DeathGoal,
    pub lose_condition: DeathGoal,
}

impl Level {
    pub fn get_current(world: &mut World) -> Level {
        let mut level = Level::get_stub(); // Does not take win condition

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
            .map(|e| world.get::<MaybeCard>(e).unwrap());

        for card in deck {
            match card.0 {
                Some(card) => {
                    level.starting_deckbar.push(card);
                }
                None => {}
            }
        }

        level
    }

    pub fn get_stub() -> Level {
        Level {
            cards: Vec::new(),
            starting_deckbar: Vec::new(),
            win_condition: DeathGoal {
                card: Card::Quakka,
                count_dead: 99,
            },
            lose_condition: DeathGoal {
                card: Card::Quakka,
                count_dead: 99,
            },
        }
    }
}

pub fn level_plugin(app: &mut App) {
    app.add_plugins(JsonAssetPlugin::<Level>::new(&["level.json"]));
}
