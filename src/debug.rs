use bevy::prelude::*;

use crate::global::CursorWorldCoords;

#[derive(Resource, PartialEq)]
pub struct IsDebug(pub bool);

impl Default for IsDebug {
    fn default() -> Self {
        if let Ok(duckslayer_debug) = std::env::var("DUCKSLAYER_DEBUG") {
            if duckslayer_debug == "true" {
                return IsDebug(true);
            } else {
                return IsDebug(false);
            }
        }
        IsDebug(false)
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
}

fn print_out_world_coords_on_click(
    world_coords: Res<CursorWorldCoords>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        dbg!(world_coords);
    }
}
