pub mod checkbox;
pub mod slider;

use bevy::prelude::*;

pub fn widgets_plugin(app: &mut App) {
    app.add_plugins(checkbox::checkbox_plugin);
}
