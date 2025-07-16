use bevy::{prelude::*, window::WindowResolution};

mod asset_load_schedule;
mod back_btn;
mod card;
mod debug;
mod debug_ui;
mod deckbar;
mod global;
mod goal_board;
mod ingame_ui_root;
mod level_select;
mod manage_level;
mod settings_screen;
mod titlescreen;
mod volume_settings;
mod widgets;

use bevy_egui::EguiPlugin;
use std::env;

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
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
        // .add_plugins(DebugPickingPlugin::default())
        // .insert_resource(DebugPickingMode::Noisy)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins((
            asset_load_schedule::asset_load_schedule,
            titlescreen::titlescreen,
            global::global,
            card::card,
            deckbar::deckbar,
            manage_level::manage_level,
            back_btn::back_btn,
            level_select::level_select,
            ingame_ui_root::ingame_ui_root_plugin,
            debug::debug_plugin,
            goal_board::goal_board_plugin,
            volume_settings::volume_settings_plugin,
            settings_screen::settings_screen_plugin,
            debug_ui::debug_ui_plugin,
            widgets::widgets_plugin,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Resolution is incorrectly found on joel's computer and is easier to manually set resolution
// instead of messing with xserver/wayland settings
fn get_resolution() -> WindowResolution {
    let mut is_joels_computer = false;
    if let Ok(joels_comuter_env) = env::var("JOELS_COMPUTER") {
        if joels_comuter_env == "true" {
            is_joels_computer = true;
        }
    }

    let Ok(current_computer) = env::var("CURRENT_COMPUTER") else {
        return default();
    };

    let mut resolution = WindowResolution::default();
    if is_joels_computer {
        let scale_factor = if current_computer == "hp" { 1.406 } else { 1.0 };

        resolution = resolution.with_scale_factor_override(scale_factor);
    }
    resolution
}
