use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
use bevy::{prelude::*, window::WindowResolution};

mod asset_load_schedule;
mod back_btn;
mod card;
mod deckbar;
mod game_messages;
mod global;
mod level_select;
mod manage_level;
mod titlescreen;

use bevy_egui::EguiPlugin;
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
        .add_plugins(DebugPickingPlugin::default())
        .insert_resource(DebugPickingMode::Noisy)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(asset_load_schedule::asset_load_schedule)
        .add_plugins(titlescreen::titlescreen)
        .add_plugins(global::global)
        .add_plugins(card::card)
        .add_plugins(deckbar::deckbar)
        .add_plugins(manage_level::manage_level)
        .add_plugins(back_btn::back_btn)
        .add_plugins(level_select::level_select)
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
        resolution = resolution.with_scale_factor_override(1.406);
    }
    resolution
}
