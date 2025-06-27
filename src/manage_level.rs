mod debug_ui;
mod editor_ui;
mod game_controls;
mod game_messages;
mod level;

use bevy::prelude::*;
pub use debug_ui::DisplayInDebug;
pub use game_messages::set_message;
pub use level::Level;
use strum::IntoEnumIterator;
use DuckSlayer::{delete_all, remove_resource};

use crate::{
    back_btn::{hide_back_btn, show_back_btn},
    card::{Card, CardDeath, SpawnCard},
    deckbar::{clear_deckbar, hide_deckbar, show_deckbar, PushToDeckbar},
    global::{not_in_editor, GameState, ImageHandles, InEditorRes},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum IsPaused {
    True,
    False,
}

#[derive(Resource, Debug)]
struct WinLoseDeathProgress {
    win: u32,
    lose: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States, Default)]
pub enum LevelProgress {
    #[default]
    Null,
    GameOver,
    GameWon,
}

#[derive(Resource, Deref, DerefMut, Debug)]
struct LevelMemory(pub Level);

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
                load_card_sprites,
                show_deckbar,
                show_back_btn,
            ),
        )
        .add_systems(
            FixedUpdate,
            (win_or_lose_on_conditions)
                .run_if(in_state(GameState::InGame).and(in_state(LevelProgress::Null))),
        )
        .add_systems(
            OnEnter(IsPaused::False),
            set_message("").run_if(not_in_editor),
        )
        .add_systems(
            OnEnter(LevelProgress::GameOver),
            (pause, set_message("You lost :(")),
        )
        .add_systems(
            OnEnter(LevelProgress::GameWon),
            (pause, set_message("You won! :)")),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (
                delete_all::<ArenaBackground>,
                delete_all::<LevelEntity>,
                clear_deckbar,
                remove_resource::<LevelMemory>,
                remove_resource::<WinLoseDeathProgress>,
                set_in_editor_false,
                reset_level_progress,
                hide_deckbar,
                hide_back_btn,
                set_message(""),
                pause,
                unload_card_sprites,
            ),
        )
        .insert_state::<IsPaused>(IsPaused::True)
        .init_resource::<CardSpriteHandles>()
        .init_state::<LevelProgress>();
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

fn save_level_to_memory(world: &mut World) {
    let current_level = Level::get_current(world);
    let mut level_res = world.get_resource_mut::<LevelMemory>().unwrap();

    level_res.0 = current_level;
}

fn spawn_entities_from_level_memory(level: Res<LevelMemory>, mut commands: Commands) {
    let level = &level.0;

    spawn_entities_from_level(&level, &mut commands);
}

fn spawn_entities_from_level(level: &Level, commands: &mut Commands) {
    commands.run_system_cached(clear_deckbar);
    commands.run_system_cached(delete_all::<LevelEntity>);

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

#[derive(Deref, DerefMut)]
pub struct EnterLevel(pub Level);
impl Command for EnterLevel {
    fn apply(self, world: &mut World) -> () {
        spawn_entities_from_level(&self, &mut world.commands());
        world
            .resource_mut::<NextState<GameState>>()
            .set(GameState::InGame);

        world.insert_resource(WinLoseDeathProgress {
            win: self.win_condition.count_dead,
            lose: self.lose_condition.count_dead,
        });

        world.insert_resource(LevelMemory(self.0));
    }
}

fn set_in_editor_false(mut in_editor: ResMut<InEditorRes>) {
    **in_editor = false;
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

fn win_or_lose_on_conditions(
    mut level_progress: ResMut<NextState<LevelProgress>>,
    mut card_death_evs: EventReader<CardDeath>,

    mut win_lose_progress: ResMut<WinLoseDeathProgress>,
    level: Res<LevelMemory>,
) {
    for card_death in card_death_evs.read() {
        if **card_death == level.win_condition.card {
            win_lose_progress.win -= 1;
        }

        if win_lose_progress.win == 0 {
            level_progress.set(LevelProgress::GameWon);
            break;
        }

        if **card_death == level.lose_condition.card {
            win_lose_progress.lose -= 1;
        }

        if win_lose_progress.lose == 0 {
            level_progress.set(LevelProgress::GameOver);
            break;
        }
    }
}

fn reset_level_progress(
    mut level_prog: ResMut<NextState<LevelProgress>>,
    level: Res<LevelMemory>,
    mut commands: Commands,
) {
    level_prog.set(LevelProgress::Null);
    commands.insert_resource(WinLoseDeathProgress {
        win: level.win_condition.count_dead,
        lose: level.lose_condition.count_dead,
    });
}
