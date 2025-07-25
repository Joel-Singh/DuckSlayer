use super::game_messages::SetMessage;
use super::{
    pause, reset_level_progress, save_level_to_memory, set_message,
    spawn_entities_from_level_memory, toggle_pause, IsPaused, LevelEntity, LevelMemory,
};
use crate::card::{MaybeCard, SpawnCard};
use crate::debug_ui::DisplayInDebug;
use crate::deckbar::{deselect_card, select_card, DeleteSelectedCard, SelectedCard};
use crate::global::{
    get_left_river_rect, get_middle_river_rect, get_right_river_rect, in_editor, not_in_editor,
    CursorWorldCoords, GameState, IsPointerOverUi,
};
use crate::manage_level::{unpause, Level};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_egui::input::egui_wants_any_keyboard_input;

pub const CONTROLS_MESSAGE: &'static str = "[Space] to start level\n[Z] to restart level\n";
pub const CONTROLS_EDITOR_MESSAGE: &'static str =
    "[Space] to toggle pausing \n[Click] on spawned cards to delete\n";

#[derive(Resource, Deref, DerefMut, Default, PartialEq)]
struct GameIsReset(bool);

/// Handles all controls for the game
pub fn game_controls_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        (
            set_starting_message.run_if(not_in_editor),
            set_message(CONTROLS_EDITOR_MESSAGE).run_if(in_editor),
            disallow_game_reset,
        ),
    )
    .add_systems(OnEnter(IsPaused::False), allow_game_reset)
    .add_systems(
        FixedPreUpdate,
        (
            (
                unpause.run_if(input_just_pressed(KeyCode::Space).and(in_state(IsPaused::True))),
                restart_level(set_starting_message.into_configs()).run_if(
                    input_just_pressed(KeyCode::KeyZ).and(resource_equals(GameIsReset(false))),
                ),
            )
                .run_if(not_in_editor),
            (
                restart_level(set_message(CONTROLS_EDITOR_MESSAGE)).run_if(
                    input_just_pressed(KeyCode::KeyZ).and(resource_equals(GameIsReset(false))),
                ),
                save_level_to_memory.run_if(input_just_pressed(KeyCode::KeyX)),
                toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
                delete_level_entities_on_click,
            )
                .run_if(in_editor),
            (
                select_card(0).run_if(input_just_pressed(KeyCode::Digit1)),
                select_card(1).run_if(input_just_pressed(KeyCode::Digit2)),
                select_card(2).run_if(input_just_pressed(KeyCode::Digit3)),
                select_card(3).run_if(input_just_pressed(KeyCode::Digit4)),
                deselect_card.run_if(input_just_pressed(KeyCode::Escape)),
                deselect_card.run_if(input_just_pressed(KeyCode::CapsLock)),
            )
                .run_if(not(egui_wants_any_keyboard_input)),
            spawn_card_on_click,
        )
            .run_if(in_state(GameState::InGame)),
    )
    .init_resource::<GameIsReset>();

    if crate::debug::in_debug() {
        app.add_systems(FixedUpdate, display_game_is_reset);
    }
}

fn restart_level(
    set_message_system: ScheduleConfigs<ScheduleSystem>,
) -> ScheduleConfigs<ScheduleSystem> {
    (
        spawn_entities_from_level_memory,
        pause,
        reset_level_progress,
        set_message_system,
        disallow_game_reset,
    )
        .chain()
}

fn set_starting_message(mut commands: Commands, level: Res<LevelMemory>) {
    let condition_string = get_condition_string(&**level);
    let starting_message = format!("{CONTROLS_MESSAGE}{condition_string}");

    commands.queue(SetMessage(starting_message));

    pub fn get_condition_string(level: &Level) -> String {
        let mut win_condition_card = level.win_condition.card.to_string();
        let win_condition_count = level.win_condition.count_dead.to_string();
        if level.win_condition.count_dead > 1 {
            win_condition_card = format!("{win_condition_card}s");
        }

        let lose_condition_card = level.lose_condition.card.to_string();
        let lose_condition_count = level.lose_condition.count_dead.to_string();
        let s = if level.lose_condition.count_dead > 1 {
            String::from("s")
        } else {
            String::from("")
        };

        format!("Eliminate {win_condition_count} {win_condition_card} and avoid {lose_condition_count} {lose_condition_card} death{s}")
    }
}

fn delete_level_entities_on_click(
    level_entities: Query<Entity, Added<LevelEntity>>,
    mut commands: Commands,
) {
    for level_entity in level_entities {
        commands.entity(level_entity).insert(Pickable::default());
        commands.entity(level_entity).observe(
            |trigger: Trigger<Pointer<Click>>,
             is_paused: Res<State<IsPaused>>,
             mut commands: Commands| {
                match **is_paused {
                    IsPaused::True => {
                        commands.entity(trigger.target()).despawn();
                    }
                    IsPaused::False => {}
                }
            },
        );
    }
}

fn spawn_card_on_click(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mouse_coords: Res<CursorWorldCoords>,
    is_pointer_over_ui: Res<IsPointerOverUi>,
    selected_card: Option<Single<&MaybeCard, With<SelectedCard>>>,
) {
    let Some(selected_card) = selected_card.map(Single::into_inner) else {
        mousebtn_evr.clear();
        return;
    };

    let Some(selected_card) = selected_card.0 else {
        return;
    };

    for ev in mousebtn_evr.read() {
        if ev.state != ButtonState::Pressed
            || **is_pointer_over_ui
            || (!selected_card.is_placeable_over_water() && !in_bounds(**mouse_coords))
        {
            continue;
        }

        commands.queue(SpawnCard::new(selected_card, mouse_coords.0));
        commands.queue(DeleteSelectedCard::default());
    }

    fn in_bounds(v: Vec2) -> bool {
        !get_left_river_rect().contains(v)
            && !get_middle_river_rect().contains(v)
            && !get_right_river_rect().contains(v)
    }
}

fn allow_game_reset(mut game_is_reset: ResMut<GameIsReset>) {
    **game_is_reset = false;
}

fn disallow_game_reset(mut game_is_reset: ResMut<GameIsReset>) {
    **game_is_reset = true;
}

fn display_game_is_reset(
    mut display_in_debug: ResMut<DisplayInDebug>,
    game_is_reset: Res<GameIsReset>,
) {
    let game_is_reset = if **game_is_reset { "true" } else { "false" };
    display_in_debug.insert("GameIsReset".to_string(), game_is_reset.to_string());
}
