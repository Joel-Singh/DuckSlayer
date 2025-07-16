use bevy::{platform::collections::HashMap, prelude::*};
use bevy_egui::{
    egui::{self},
    EguiContextPass, EguiContexts,
};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DisplayInDebug(HashMap<String, String>);

pub fn debug_ui_plugin(app: &mut App) {
    if crate::debug::in_debug() {
        app.add_systems(EguiContextPass, create_debug_window)
            .init_resource::<DisplayInDebug>();
    }
}

fn create_debug_window(mut contexts: EguiContexts, display_in_debug: Res<DisplayInDebug>) {
    egui::Window::new("DEBUG UI").show(contexts.ctx_mut(), |ui| {
        for (k, v) in &**display_in_debug {
            ui.label(&format!("{k}: {v}"));
        }
    });
}
