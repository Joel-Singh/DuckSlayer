mod debug_ui;
mod editor_ui;
mod game_messages;
mod level;

use bevy::{
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use game_messages::set_message;
use level::Level;
use DuckSlayer::delete_all;

use crate::{
    back_btn::{hide_back_btn, show_back_btn},
    card::{spawn_card, Bridge, NestDestroyed},
    deckbar::{clear_deckbar, hide_deckbar, show_deckbar, PushToDeckbar},
    global::{in_editor, not_in_editor, GameState, ImageHandles, IsInEditor, BRIDGE_LOCATIONS},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum IsPaused {
    True,
    False,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States, Default)]
pub enum GameOver {
    True,
    #[default]
    False,
}

#[derive(Resource, Default)]
struct LevelRes(Level);

#[derive(Component, Default)]
pub struct LevelEntity;

#[derive(Component)]
struct ArenaBackground;

pub fn manage_level(app: &mut App) {
    app.add_plugins(game_messages::game_messages)
        .add_plugins(debug_ui::debug_ui_plugin)
        .add_plugins(editor_ui::editor_ui_plugin)
        .add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_arena_background,
                spawn_bridge_locations,
                load_from_level_res().run_if(not_in_editor),
                show_deckbar,
                set_message("[Space] to start level").run_if(not_in_editor),
                set_message("[Space] to toggle pausing \n[Click] on spawned cards to delete\n[Z] to load level from memory")
                    .run_if(in_editor),
                show_back_btn,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                (
                    (
                        load_from_level_res(),
                        pause,
                        set_gameover_false,
                        set_message("[Space] to start level"),
                    )
                        .chain()
                        .run_if(input_just_pressed(KeyCode::KeyZ)),
                    gameover_on_nest_destruction,
                    unpause
                        .run_if(input_just_pressed(KeyCode::Space).and(in_state(GameOver::False))),
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
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            OnEnter(IsPaused::False),
            set_message("").run_if(not_in_editor),
        )
        .add_systems(
            OnEnter(GameOver::True),
            (pause, set_message("Gameover: nest destroyed")),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (
                delete_all::<ArenaBackground>,
                delete_all::<Bridge>,
                delete_all::<LevelEntity>,
                set_in_editor_false,
                hide_deckbar,
                hide_back_btn,
                set_message(""),
                pause,
            ),
        )
        .insert_state::<IsPaused>(IsPaused::True)
        .init_state::<GameOver>()
        .init_resource::<LevelRes>();
}

fn spawn_arena_background(mut commands: Commands, image_handles: Res<ImageHandles>) {
    commands.spawn((
        Sprite {
            image: image_handles.arena_background.clone(),
            ..default()
        },
        Transform {
            // -0.5 so it's in the back and clicks are registered to Nodes
            translation: Vec3::new(0., 0., -0.5),
            ..default()
        },
        ArenaBackground,
    ));
}

fn spawn_bridge_locations(mut commands: Commands) {
    commands.spawn((
        Bridge,
        Transform {
            translation: BRIDGE_LOCATIONS.0.extend(0.0),
            ..default()
        },
    ));

    commands.spawn((
        Bridge,
        Transform {
            translation: BRIDGE_LOCATIONS.1.extend(0.0),
            ..default()
        },
    ));
}

fn save_level_to_resource(world: &mut World) {
    let current_level = Level::get_current(world);
    let mut level_res = world.get_resource_mut::<LevelRes>().unwrap();

    level_res.0 = current_level;
}

fn delete_level_entities_on_click(
    level_entities: Query<Entity, Added<LevelEntity>>,
    mut commands: Commands,
) {
    for level_entity in level_entities {
        commands.entity(level_entity).insert(Pickable::default());
        commands.entity(level_entity).observe(
            |trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                commands.entity(trigger.target()).despawn();
            },
        );
    }
}

fn load_from_level_res() -> ScheduleConfigs<ScheduleSystem> {
    return (
        clear_deckbar,
        delete_all::<LevelEntity>,
        spawn_entities_from_level_res,
    )
        .chain();
}

fn spawn_entities_from_level_res(
    level: Res<LevelRes>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let level = &level.0;

    for (card, position) in &level.cards {
        spawn_card(*card, *position, &mut commands, &asset_server);
    }

    for card in &level.starting_deckbar {
        commands.queue(PushToDeckbar(*card));
    }
}

fn unpause(mut is_paused: ResMut<NextState<IsPaused>>) {
    is_paused.set(IsPaused::False);
}

fn pause(mut is_paused: ResMut<NextState<IsPaused>>) {
    is_paused.set(IsPaused::True);
}

struct Pause;
impl Command for Pause {
    fn apply(self, world: &mut World) -> () {
        let mut is_paused = world.get_resource_mut::<NextState<IsPaused>>().unwrap();
        is_paused.set(IsPaused::True);
    }
}

fn set_in_editor_false(mut is_in_editor: ResMut<NextState<IsInEditor>>) {
    is_in_editor.set(IsInEditor::False);
}

fn toggle_pause(mut is_paused_mut: ResMut<NextState<IsPaused>>, is_paused: Res<State<IsPaused>>) {
    match **is_paused {
        IsPaused::True => {
            is_paused_mut.set(IsPaused::False);
        }
        IsPaused::False => {
            is_paused_mut.set(IsPaused::True);
        }
    }
}

fn gameover_on_nest_destruction(
    mut gameover: ResMut<NextState<GameOver>>,
    mut nest_destroyed_evs: EventReader<NestDestroyed>,
) {
    for _ in nest_destroyed_evs.read() {
        gameover.set(GameOver::True);
    }
}

fn set_gameover_false(mut gameover: ResMut<NextState<GameOver>>) {
    gameover.set(GameOver::False);
}
