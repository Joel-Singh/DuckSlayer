use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContextPass, EguiContexts,
};
use strum::IntoEnumIterator;

use crate::{
    card::{Card, IsSpawnedCardDebugOverlayEnabled},
    deckbar::PushToDeckbar,
    global::{in_debug, GameState},
};

use super::{GameOver, IsPaused};

pub fn debug_ui_plugin(app: &mut App) {
    app.add_systems(EguiContextPass, create_debug_window.run_if(in_debug));
}

fn create_debug_window(
    mut contexts: EguiContexts,
    mut commands: Commands,

    game_over: Res<State<GameOver>>,
    is_paused: Res<State<IsPaused>>,
    game_state: Res<State<GameState>>,
) {
    egui::Window::new("DEBUG UI").show(contexts.ctx_mut(), |ui| {
        create_push_to_deckbar_btns(ui, &mut commands);
        if ui.button("Toggle Spawned Card Debug Overlay").clicked() {
            commands.queue(move |world: &mut World| {
                let mut is_overlay_enabled = world
                    .get_resource_mut::<IsSpawnedCardDebugOverlayEnabled>()
                    .unwrap();

                is_overlay_enabled.0 = !is_overlay_enabled.0;
            })
        }

        ui.heading("Resources");
        ui.label("Gameover: ".to_string() + &format!("{game_over:?}"));

        ui.label("IsPaused: ".to_string() + &format!("{is_paused:?}"));

        ui.label("GameState: ".to_string() + &format!("{game_state:?}"));
    });
}

fn create_push_to_deckbar_btns(ui: &mut Ui, commands: &mut Commands) {
    for card in Card::iter() {
        let push_to_deck_btn = ui.button("Push ".to_string() + &card.to_string());
        if push_to_deck_btn.clicked() {
            commands.queue(PushToDeckbar(card));
        }
    }
}
