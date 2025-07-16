use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::global::CursorWorldCoords;

pub fn in_debug() -> bool {
    if let Ok(duckslayer_debug) = std::env::var("DUCKSLAYER_DEBUG") {
        duckslayer_debug == "true"
    } else {
        false
    }
}

pub fn debug_plugin(app: &mut App) {
    if in_debug() {
        app.add_systems(FixedUpdate, print_out_world_coords_on_click)
            .add_plugins((
                FpsOverlayPlugin {
                    config: FpsOverlayConfig {
                        enabled: true,
                        ..default()
                    },
                },
                WorldInspectorPlugin::new(),
            ));
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
