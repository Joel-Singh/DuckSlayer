use bevy::{prelude::*, window::WindowResolution};

mod deckbar;
mod game_messages;
mod global;
mod manage_level;
mod titlescreen;
mod troops;

use deckbar::*;
use global::*;
use manage_level::*;
use titlescreen::*;
use troops::*;

use std::env;

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
                        resolution: get_resolution(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(titlescreen)
        .add_plugins(troops)
        .add_plugins(global)
        .add_plugins(deckbar)
        .add_plugins(manage_level)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Resolution is incorrectly found on joel's computer
fn get_resolution() -> WindowResolution {
    let mut is_joels_computer = false;
    if let Ok(joels_comuter_env) = env::var("JOELS_COMPUTER") {
        if joels_comuter_env == "true" {
            is_joels_computer = true;
        }
    }

    let mut resolution = WindowResolution::default();
    if is_joels_computer {
        resolution = resolution.with_scale_factor_override(1.0);
    }
    resolution
}
