use bevy::{color::palettes::css::BLUE, platform::collections::HashMap, prelude::*};
use bevy_egui::{
    egui::{self, Ui},
    EguiContextPass, EguiContexts,
};
use strum::IntoEnumIterator;

use crate::{
    card::Card,
    debug::in_debug,
    deckbar::PushToDeckbar,
    global::{
        get_left_river_rect, get_middle_river_rect, get_right_river_rect, GameState,
        IsPointerOverUi,
    },
};

use super::{IsPaused, LevelMemory, LevelProgress, WinLoseDeathProgress};

const DRAW_RIVER_BOUNDARIES: bool = true;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DisplayInDebug(HashMap<String, String>);

pub fn debug_ui_plugin(app: &mut App) {
    app.add_systems(
        EguiContextPass,
        (create_debug_window, draw_river_boundaries).run_if(in_debug),
    )
    .init_resource::<DisplayInDebug>();
}

fn create_debug_window(
    mut contexts: EguiContexts,
    mut commands: Commands,

    display_in_debug: Res<DisplayInDebug>,

    level_progress: Res<State<LevelProgress>>,
    is_paused: Res<State<IsPaused>>,
    game_state: Res<State<GameState>>,
    is_pointer_over_ui: Res<IsPointerOverUi>,
    win_lose_death_progress: Option<Res<WinLoseDeathProgress>>,
    level_memory: Option<Res<LevelMemory>>,
) {
    egui::Window::new("DEBUG UI").show(contexts.ctx_mut(), |ui| {
        create_push_to_deckbar_btns(ui, &mut commands);

        ui.heading("Resources");
        ui.label(&format!("LevelProgress: {level_progress:?}"));
        ui.label(&format!("IsPaused: {is_paused:?}"));
        ui.label(&format!("GameState: {game_state:?}"));
        ui.label(&format!("IsPointerOverUi: {is_pointer_over_ui:?}"));
        ui.label(format!("WinLoseDeathProgress: {win_lose_death_progress:?}"));
        for (k, v) in &**display_in_debug {
            ui.label(&format!("{k}: {v}"));
        }

        ui.collapsing("Level in memory", |ui| {
            ui.label(format!("{level_memory:?}"))
        });
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

fn draw_river_boundaries(mut draw: Gizmos) {
    for rect in [
        get_left_river_rect(),
        get_right_river_rect(),
        get_middle_river_rect(),
    ] {
        if !DRAW_RIVER_BOUNDARIES {
            return;
        }

        draw.rect_2d(
            Isometry2d::from_translation(rect.center()),
            rect.size(),
            BLUE,
        );
    }
}
