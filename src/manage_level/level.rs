use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, MaybeCard, SpawnedCard},
    deckbar::DeckBarRoot,
};

use super::LevelMemory;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeathGoal {
    pub card: Card,
    pub count_dead: u32,
}

#[derive(Serialize, Deserialize, Asset, TypePath, Debug)]
pub struct Level {
    pub cards: Vec<(Card, Vec2)>,
    pub starting_deckbar: Vec<Card>,
    pub win_condition: DeathGoal,
    pub lose_condition: DeathGoal,
}

impl Level {
    pub fn get_current(world: &mut World) -> Level {
        let mut current_level = Level::get_stub();

        let mut cards = world.query::<(&Transform, &SpawnedCard)>();
        for (transform, spawned_card) in cards.iter(world) {
            if **spawned_card == Card::Waterball {
                // It doesn't make sense to save waterballs
                continue;
            }
            current_level
                .cards
                .push((**spawned_card, transform.translation.truncate()));
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
                    current_level.starting_deckbar.push(card);
                }
                None => {}
            }
        }

        let level_in_memory = world.get_resource::<LevelMemory>().unwrap();
        current_level.win_condition = level_in_memory.win_condition.clone();
        current_level.lose_condition = level_in_memory.lose_condition.clone();

        current_level
    }

    pub fn get_stub() -> Level {
        Level {
            cards: Vec::new(),
            starting_deckbar: Vec::new(),
            win_condition: DeathGoal {
                card: Card::Quakka,
                count_dead: 1,
            },
            lose_condition: DeathGoal {
                card: Card::Quakka,
                count_dead: 1,
            },
        }
    }
}

pub fn level_plugin(app: &mut App) {
    app.add_plugins(JsonAssetPlugin::<Level>::new(&["level.json"]));
}
