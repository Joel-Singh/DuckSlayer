use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::WindowResolution};

mod deckbar;
mod game_messages;
mod global;
mod manage_level;
mod titlescreen;
mod troops;

use deckbar::*;
use game_messages::*;
use global::*;
use manage_level::*;
use titlescreen::*;
use troops::*;

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
        .add_plugins(manage_level)
        .add_plugins(game_messages)
        .add_systems(Startup, setup_camera)
        .add_systems(
            FixedUpdate,
            unpause.run_if(input_just_pressed(KeyCode::Space).and(in_state(GameState::InGame))),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn unpause(mut is_paused: ResMut<NextState<IsPaused>>) {
    is_paused.set(IsPaused::False);
}
