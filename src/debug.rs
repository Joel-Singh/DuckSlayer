use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

use crate::global::CursorWorldCoords;

fn get_debug_env_var() -> bool {
    if let Ok(duckslayer_debug) = std::env::var("DUCKSLAYER_DEBUG") {
        duckslayer_debug == "true"
    } else {
        false
    }
}

#[derive(Resource, PartialEq)]
pub struct IsDebug(pub bool);

impl Default for IsDebug {
    fn default() -> Self {
        IsDebug(get_debug_env_var())
    }
}

pub fn in_debug(is_debug: Res<IsDebug>) -> bool {
    is_debug.0
}

pub fn debug_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (print_out_world_coords_on_click).run_if(in_debug),
    )
    .init_resource::<IsDebug>();

    if get_debug_env_var() {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                enabled: true,
                ..default()
            },
        });
    };
}

fn print_out_world_coords_on_click(
    world_coords: Res<CursorWorldCoords>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        dbg!(world_coords);
    }
}
