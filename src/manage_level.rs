mod debug_ui;
mod editor_ui;
mod game_controls;
mod game_messages;
mod level;

use bevy::{
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    prelude::*,
};
pub use game_messages::set_message;
pub use level::Level;
use strum::IntoEnumIterator;
use DuckSlayer::delete_all;

use crate::{
    back_btn::{hide_back_btn, show_back_btn},
    card::{Bridge, Card, NestDestroyed, SpawnCard},
    deckbar::{clear_deckbar, hide_deckbar, show_deckbar, PushToDeckbar},
    global::{not_in_editor, GameState, ImageHandles, IsInEditor, BRIDGE_LOCATIONS},
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
pub struct LevelRes(pub Level);

#[derive(Component, Default)]
pub struct LevelEntity;

#[derive(Component)]
struct ArenaBackground;

#[derive(Resource, Default)]
struct CardSpriteHandles(Vec<Handle<Image>>);

pub fn manage_level(app: &mut App) {
    app.add_plugins(game_messages::game_messages)
        .add_plugins(debug_ui::debug_ui_plugin)
        .add_plugins(editor_ui::editor_ui_plugin)
        .add_plugins(level::level_plugin)
        .add_plugins(game_controls::game_controls_plugin)
        .add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_arena_background,
                spawn_bridge_locations,
                load_from_level_res().run_if(not_in_editor),
                load_card_sprites,
                show_deckbar,
                show_back_btn,
            ),
        )
        .add_systems(
            FixedUpdate,
            (gameover_on_nest_destruction.run_if(not_in_editor))
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
                set_gameover_false,
                hide_deckbar,
                hide_back_btn,
                set_message(""),
                pause,
                unload_card_sprites,
            ),
        )
        .insert_state::<IsPaused>(IsPaused::True)
        .init_resource::<CardSpriteHandles>()
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

fn load_card_sprites(
    mut card_sprite_handles: ResMut<CardSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    for card in Card::iter() {
        card_sprite_handles
            .0
            .push(asset_server.load(card.get_sprite_filepath()));
    }
}

fn unload_card_sprites(mut card_sprite_handles: ResMut<CardSpriteHandles>) {
    card_sprite_handles.0.clear();
}

fn load_from_level_res() -> ScheduleConfigs<ScheduleSystem> {
    return (
        clear_deckbar,
        delete_all::<LevelEntity>,
        spawn_entities_from_level_res,
    )
        .chain();
}

fn spawn_entities_from_level_res(level: Res<LevelRes>, mut commands: Commands) {
    let level = &level.0;

    for (card, position) in &level.cards {
        commands.queue(SpawnCard::new(*card, *position));
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
