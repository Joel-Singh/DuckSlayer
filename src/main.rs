use bevy::{prelude::*, window::WindowResolution};

use std::time::Duration;

mod deckbar;
mod global;
mod titlescreen;
mod troops;

use deckbar::*;
use global::*;
use titlescreen::*;
use troops::*;

const ARENA_WIDTH: f32 = 0.8 * SCREEN_WIDTH;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(bevy::log::LogPlugin {
                    //level: bevy::log::Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::default().with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(titlescreen)
        .add_plugins(troops)
        .add_plugins(global)
        .add_plugins(deckbar)
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::InGame), spawn_entities)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_entities(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Sprite {
            image: asset_server.load("quakka.png"),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 200., 0.),
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
    ));

    spawn_nest(
        Vec3::new(
            0. - DECK_WIDTH - 0.15 * ARENA_WIDTH,
            0.0 - 0.25 * SCREEN_HEIGHT,
            0.,
        ),
        &mut commands,
        &asset_server,
    );

    spawn_nest(
        Vec3::new(
            0. - DECK_WIDTH + 0.15 * ARENA_WIDTH,
            0.0 - 0.25 * SCREEN_HEIGHT,
            0.,
        ),
        &mut commands,
        &asset_server,
    );

    commands.spawn((
        Bridge,
        Transform {
            translation: Vec3::new(
                0. - DECK_WIDTH - 0.15 * ARENA_WIDTH,
                0.0 + 0.25 * SCREEN_HEIGHT,
                0.,
            ),
            ..default()
        },
    ));
}

fn spawn_nest(translation: Vec3, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("nest.png"),
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        Transform {
            translation,
            ..default()
        },
        Health {
            current_health: 100.0,
            max_health: 100.0,
            healthbar_height: 60.,
        },
        Nest,
    ));
}
