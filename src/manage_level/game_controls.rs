use crate::deckbar::{deselect_card, select_card};
use crate::global::{in_editor, not_in_editor, GameState};
use crate::manage_level::load_from_level_res;
use crate::manage_level::unpause;
use crate::manage_level::GameOver;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use super::{pause, set_gameover_false, set_message, toggle_pause, IsPaused, LevelEntity};

pub fn game_controls_plugin(app: &mut App) {
    let start_msg: &'static str = "[Space] to start level\n[Z] to restart level";

    app.add_systems(
        OnEnter(GameState::InGame),
        (
            set_message(start_msg).run_if(not_in_editor),
            set_message("[Space] to toggle pausing \n[Click] on spawned cards to delete")
                .run_if(in_editor),
        ),
    )
    .add_systems(
        FixedUpdate,
        (
            (
                unpause.run_if(input_just_pressed(KeyCode::Space).and(in_state(GameOver::False))),
                (
                    load_from_level_res(),
                    pause,
                    set_gameover_false,
                    set_message(start_msg),
                )
                    .chain()
                    .run_if(input_just_pressed(KeyCode::KeyZ)),
            )
                .run_if(not_in_editor),
            (
                (load_from_level_res(), pause)
                    .chain()
                    .run_if(input_just_pressed(KeyCode::KeyZ)),
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
            ),
        )
            .run_if(in_state(GameState::InGame)),
    );
}

fn delete_level_entities_on_click(
    level_entities: Query<Entity, Added<LevelEntity>>,
    mut commands: Commands,
) {
    for level_entity in level_entities {
        commands.entity(level_entity).insert(Pickable::default());
        commands.entity(level_entity).observe(
            |trigger: Trigger<Pointer<Click>>, is_paused: Res<State<IsPaused>>, mut commands: Commands| {
                match **is_paused {
                    IsPaused::True => {commands.entity(trigger.target()).despawn();}
                    IsPaused::False => {}
                }
            },
        );
    }
}
