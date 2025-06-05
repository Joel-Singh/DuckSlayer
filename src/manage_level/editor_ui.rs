use bevy::{ecs::system::RunSystemOnce, prelude::*};
use bevy_egui::{
    egui::{self, Ui},
    EguiContextPass, EguiContexts,
};
use strum::IntoEnumIterator;
use DuckSlayer::delete_all;

use crate::{
    card::{spawn_card, Card},
    deckbar::{clear_deckbar, PushToDeckbar},
    global::{in_editor, NEST_POSITIONS},
};

use super::{pause, save_level, spawn_entities_from_level, LevelEntity};

pub fn editor_ui_plugin(app: &mut App) {
    app.add_systems(EguiContextPass, create_editor_window.run_if(in_editor));
}

fn create_editor_window(mut contexts: EguiContexts, mut commands: Commands) {
    egui::Window::new("Editor")
        .default_pos((0., 160.)) // Stop from spawning ontop of back btn
        .show(contexts.ctx_mut(), |ui| {
            create_push_to_deckbar_btns(ui, &mut commands);

            if ui.button("Spawn nests in default positions").clicked() {
                commands.queue(move |world: &mut World| {
                    let _ = world.run_system_once(spawn_nests_in_default_positions);
                })
            }

            if ui.button("Save Level to memory").clicked() {
                commands.queue(move |world: &mut World| {
                    let _ = world.run_system_once(save_level);
                })
            }

            if ui.button("Load level from memory").clicked() {
                commands.queue(move |world: &mut World| {
                    let _ = world.run_system_once(delete_all::<LevelEntity>);
                    let _ = world.run_system_once(clear_deckbar);
                    let _ = world.run_system_once(spawn_entities_from_level);
                    let _ = world.run_system_once(pause);
                })
            }
        });
}

fn create_push_to_deckbar_btns(ui: &mut Ui, commands: &mut Commands) {
    for card in Card::iter() {
        if card.is_empty() {
            continue;
        }

        let push_to_deck_btn =
            ui.button("Add ".to_string() + &card.to_string() + &" to the deck".to_string());
        if push_to_deck_btn.clicked() {
            commands.queue(PushToDeckbar(card));
        }
    }
}

fn spawn_nests_in_default_positions(mut commands: Commands, asset_server: Res<AssetServer>) {
    for pos in [NEST_POSITIONS.0, NEST_POSITIONS.1] {
        spawn_card(Card::Nest, pos.into(), &mut commands, &asset_server);
    }
}
