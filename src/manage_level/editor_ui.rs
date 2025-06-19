mod saving_loading_levels;

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use bevy_egui::{
    egui::{self, Slider, Ui},
    EguiContextPass, EguiContexts,
};
use saving_loading_levels::{
    saving_loading_levels_plugin, LoadCardConstsWithFileDialog, LoadLevelWithFileDialog,
    SaveCardConstsWithFileDialog, SaveLevelWithFileDialog,
};
use strum::IntoEnumIterator;
use DuckSlayer::delete_all;

use crate::{
    card::{Card, CardConsts, SpawnCard},
    deckbar::{clear_deckbar, PushToDeckbar},
    global::{in_editor, GameState, NEST_POSITIONS},
};

use super::{pause, save_level_to_memory, spawn_entities_from_level_memory, LevelEntity, Pause};

#[derive(Resource, Default)]
struct IsConstantsWindowOpen(bool);

pub fn editor_ui_plugin(app: &mut App) {
    app.add_plugins(saving_loading_levels_plugin)
        .add_systems(EguiContextPass, create_editor_window.run_if(in_editor))
        .add_systems(OnExit(GameState::InGame), cleanup)
        .init_resource::<IsConstantsWindowOpen>();
}

fn create_editor_window(
    mut contexts: EguiContexts,
    mut card_consts: ResMut<CardConsts>,
    mut is_constants_window_open: ResMut<IsConstantsWindowOpen>,
    mut commands: Commands,
) {
    egui::Window::new("Editor")
        .default_pos((0., 160.)) // Stop from spawning ontop of back btn
        .show(contexts.ctx_mut(), |ui| {
            ui.collapsing("Add cards", |ui| {
                create_push_to_deckbar_btns(ui, &mut commands);
            });

            if ui.button("Spawn nests in default positions").clicked() {
                commands.queue(move |world: &mut World| {
                    let _ = world.run_system_once(spawn_nests_in_default_positions);
                })
            }

            ui.heading("Quick Saving");
            if ui.button("Quicksave [X]").clicked() {
                commands.queue(move |world: &mut World| {
                    let _ = world.run_system_once(save_level_to_memory);
                })
            }

            if ui.button("Load quicksave [Z]").clicked() {
                commands.queue(move |world: &mut World| {
                    let _ = world.run_system_once(delete_all::<LevelEntity>);
                    let _ = world.run_system_once(clear_deckbar);
                    let _ = world.run_system_once(spawn_entities_from_level_memory);
                    let _ = world.run_system_once(pause);
                })
            }

            ui.heading("Saving to file");
            if ui.button("Save level to file").clicked() {
                commands.queue(SaveLevelWithFileDialog);
                commands.queue(Pause);
            }

            if ui.button("Load level from file").clicked() {
                commands.queue(Pause);
                commands.queue(LoadLevelWithFileDialog);
            }

            ui.heading("Toggles");
            if ui.button("Toggle constants window").clicked() {
                is_constants_window_open.0 = !is_constants_window_open.0;
            }
        });

    if is_constants_window_open.0 {
        egui::Window::new("Constants Editor")
            .enabled(is_constants_window_open.0)
            .default_pos((0., 400.))
            .show(contexts.ctx_mut(), |ui| {
                let const_edit =
                    |ui: &mut Ui, desc: &'static str, constant: &mut f32, max_val: f32| {
                        ui.add(Slider::new(constant, 0.0..=max_val).text(desc));
                    };

                ui.heading("Waterball");
                const_edit(ui, "Radius", &mut card_consts.waterball.radius, 250.);
                const_edit(ui, "Damage", &mut card_consts.waterball.damage, 250.);

                ui.heading("Nest");
                const_edit(ui, "Damage", &mut card_consts.nest.damage, 100.);
                const_edit(ui, "Range", &mut card_consts.nest.range, 1000.);

                if ui.button("Save current constants to file").clicked() {
                    commands.queue(Pause);
                    commands.queue(SaveCardConstsWithFileDialog);
                }

                if ui.button("Load constants from file").clicked() {
                    commands.queue(Pause);
                    commands.queue(LoadCardConstsWithFileDialog);
                }
            });
    }
}

fn create_push_to_deckbar_btns(ui: &mut Ui, commands: &mut Commands) {
    for card in Card::iter() {
        let push_to_deck_btn =
            ui.button("Add ".to_string() + &card.to_string() + &" to the deck".to_string());
        if push_to_deck_btn.clicked() {
            commands.queue(PushToDeckbar(card));
        }
    }
}

fn spawn_nests_in_default_positions(mut commands: Commands) {
    for pos in [NEST_POSITIONS.0, NEST_POSITIONS.1] {
        commands.queue(SpawnCard::new(Card::Nest, pos.into()));
    }
}

fn cleanup(
    mut is_constants_window_open: ResMut<IsConstantsWindowOpen>,
    mut card_consts: ResMut<CardConsts>,
) {
    is_constants_window_open.0 = false;
    *card_consts = CardConsts::default();
}
