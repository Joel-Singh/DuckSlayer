mod card_behaviors;
mod card_constants;

use crate::global::{NEST_DAMAGE, QUAKKA_DAMAGE};
use bevy::prelude::*;

use card_behaviors::{Attacker, GoingToBridge, Health};

pub use card_behaviors::{
    Bridge, Farmer, IsSpawnedCardDebugOverlayEnabled, Nest, NestDestroyed, Quakka, Waterball,
};
pub use card_constants::CardConsts;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use strum_macros::EnumIter;

#[derive(Component, Clone, Copy, Debug, EnumIter, Serialize, Deserialize)]
pub enum Card {
    Empty,
    Farmer,
    Quakka,
    Waterball,
    Nest,
}

pub fn card(app: &mut App) {
    app.add_plugins(card_behaviors::card_behaviors)
        .add_plugins(card_constants::card_constants);
}

impl Card {
    pub fn is_empty(&self) -> bool {
        match self {
            Card::Empty => true,
            _ => false,
        }
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn to_string(&self) -> String {
        match self {
            Card::Empty => "Empty".to_string(),
            Card::Farmer => "Farmer".to_string(),
            Card::Quakka => "Quakka".to_string(),
            Card::Waterball => "Waterball".to_string(),
            Card::Nest => "Nest".to_string(),
        }
    }

    pub fn get_sprite_size(&self, consts: &CardConsts) -> (f32, f32) {
        match self {
            Card::Empty => {
                panic!()
            }
            Card::Farmer => consts.farmer.size,
            Card::Quakka => consts.quakka.size,
            Card::Waterball => consts.waterball.size(),
            Card::Nest => consts.nest.size,
        }
    }

    pub fn get_sprite_filepath(&self) -> String {
        match self {
            Card::Empty => {
                panic!();
            }
            Card::Farmer => "farmer.png".to_string(),
            Card::Quakka => "quakka.png".to_string(),
            Card::Waterball => "waterball.png".to_string(),
            Card::Nest => "nest.png".to_string(),
        }
    }

    pub fn get_sprite(&self, asset_server: &AssetServer, card_consts: &CardConsts) -> Sprite {
        Sprite {
            image: asset_server.load(self.get_sprite_filepath()),
            custom_size: Some(self.get_sprite_size(card_consts).into()),
            ..default()
        }
    }
}

pub struct SpawnCard {
    card: Card,
    position: Vec2,
}

impl SpawnCard {
    pub fn new(card: Card, position: Vec2) -> SpawnCard {
        SpawnCard { card, position }
    }
}

impl Command for SpawnCard {
    fn apply(self, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        let card_consts = world.resource::<CardConsts>();

        match self.card {
            Card::Farmer => {
                world.spawn(farmer_bundle(self.position, asset_server, card_consts));
            }
            Card::Quakka => {
                world.spawn(quakka_bundle(self.position, asset_server, card_consts));
            }
            Card::Waterball => {
                world.spawn(waterball_bundle(self.position, asset_server, card_consts));
            }
            Card::Nest => {
                world.spawn(nest_bundle(self.position, asset_server, card_consts));
            }
            Card::Empty => warn!("Cannot spawn an empty card bundle"),
        }
    }
}

fn quakka_bundle(
    position: Vec2,
    asset_server: &AssetServer,
    card_consts: &CardConsts,
) -> impl Bundle {
    (
        Card::Quakka.get_sprite(asset_server, card_consts),
        Transform {
            translation: position.extend(0.0),
            ..default()
        },
        Health {
            current_health: 100.0,
            max_health: 100.0,
            healthbar_height: 60.,
        },
        Quakka,
        Attacker {
            cooldown: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
            damage: QUAKKA_DAMAGE,
        },
    )
}

fn farmer_bundle(
    position: Vec2,
    asset_server: &AssetServer,
    card_consts: &CardConsts,
) -> impl Bundle {
    (
        Card::Farmer.get_sprite(asset_server, card_consts),
        Transform {
            translation: position.extend(0.),
            ..default()
        },
        Farmer,
        GoingToBridge,
        Health {
            current_health: 100.0,
            max_health: 100.0,
            healthbar_height: 60.,
        },
    )
}

fn waterball_bundle(
    position: Vec2,
    asset_server: &AssetServer,
    card_consts: &CardConsts,
) -> impl Bundle {
    (
        Card::Waterball.get_sprite(asset_server, card_consts),
        Waterball {
            radius: card_consts.waterball.radius,
        },
        Transform {
            translation: position.extend(0.0),
            ..default()
        },
        Attacker {
            cooldown: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once),
            damage: card_consts.waterball.damage,
        },
    )
}

fn nest_bundle(
    position: Vec2,
    asset_server: &AssetServer,
    card_consts: &CardConsts,
) -> impl Bundle {
    (
        Card::Nest.get_sprite(asset_server, card_consts),
        Transform {
            translation: position.extend(0.0),
            ..default()
        },
        Health {
            current_health: 100.0,
            max_health: 100.0,
            healthbar_height: 60.,
        },
        Nest::default(),
        Attacker {
            cooldown: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
            damage: NEST_DAMAGE,
        },
    )
}
