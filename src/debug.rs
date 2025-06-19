use bevy::prelude::*;

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
    app.init_resource::<IsDebug>();
}
