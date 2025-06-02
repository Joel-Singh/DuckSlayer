mod card_behaviors;

use crate::global::{
    FARMER_SIZE, NEST_DAMAGE, NEST_SIZE, QUAKKA_DAMAGE, QUAKKA_SIZE, WATERBALL_DAMAGE,
    WATERBALL_SIZE,
};
use bevy::prelude::*;

use card_behaviors::{Attacker, GoingToBridge, Health};

pub use card_behaviors::{
    Bridge, Farmer, IsSpawnedCardDebugOverlayEnabled, Nest, NestDestroyed, Quakka, Waterball,
};
use std::time::Duration;
use strum_macros::EnumIter;

#[derive(Component, Clone, Copy, Debug, EnumIter)]
pub enum Card {
    Empty,
    Farmer,
    Quakka,
    Waterball,
    Nest,
}

pub fn plugin(app: &mut App) {
    app.add_plugins(card_behaviors::plugin);
}

impl Card {
    pub fn is_empty(&self) -> bool {
        match self {
            Card::Empty => true,
            _ => false,
        }
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

    pub fn get_sprite_size(&self) -> (f32, f32) {
        match self {
            Card::Empty => {
                panic!()
            }
            Card::Farmer => FARMER_SIZE,
            Card::Quakka => QUAKKA_SIZE,
            Card::Waterball => WATERBALL_SIZE,
            Card::Nest => NEST_SIZE,
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

    pub fn get_sprite(&self, asset_server: &Res<AssetServer>) -> Sprite {
        Sprite {
            image: asset_server.load(self.get_sprite_filepath()),
            custom_size: Some(self.get_sprite_size().into()),
            ..default()
        }
    }
}

pub fn spawn_card(
    card: Card,
    position: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    match card {
        Card::Farmer => {
            commands.spawn(farmer_bundle(position, asset_server));
        }
        Card::Quakka => {
            commands.spawn(quakka_bundle(position, asset_server));
        }
        Card::Waterball => {
            commands.spawn(waterball_bundle(position, asset_server));
        }
        Card::Nest => {
            commands.spawn(nest_bundle(position, asset_server));
        }
        Card::Empty => warn!("Cannot spawn an empty card bundle"),
    }

    fn quakka_bundle(position: Vec2, asset_server: &Res<AssetServer>) -> impl Bundle {
        (
            Card::Quakka.get_sprite(asset_server),
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

    fn farmer_bundle(position: Vec2, asset_server: &Res<AssetServer>) -> impl Bundle {
        (
            Card::Farmer.get_sprite(asset_server),
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

    fn waterball_bundle(position: Vec2, asset_server: &Res<AssetServer>) -> impl Bundle {
        (
            Card::Waterball.get_sprite(asset_server),
            Waterball,
            Transform {
                translation: position.extend(0.0),
                ..default()
            },
            Attacker {
                cooldown: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once),
                damage: WATERBALL_DAMAGE,
            },
        )
    }

    fn nest_bundle(position: Vec2, asset_server: &Res<AssetServer>) -> impl Bundle {
        (
            Card::Nest.get_sprite(asset_server),
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
}
