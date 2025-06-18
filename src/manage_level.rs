mod debug_ui;
mod editor_ui;
mod game_controls;
mod game_messages;
mod level;

use bevy::prelude::*;
pub use game_messages::set_message;
pub use level::Level;
use strum::IntoEnumIterator;
use DuckSlayer::delete_all;

use crate::{
    back_btn::{hide_back_btn, show_back_btn},
    card::{Bridge, Card, CardDeath, Quakka, SpawnCard},
    deckbar::{clear_deckbar, hide_deckbar, show_deckbar, PushToDeckbar},
    global::{not_in_editor, GameState, ImageHandles, IsInEditor, BRIDGE_LOCATIONS},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum IsPaused {
    True,
    False,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States, Default)]
pub enum LevelProgress {
    #[default]
    Null,
    GameOver,
    GameWon,
}

#[derive(Resource, Default, Deref, DerefMut)]
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
                spawn_bridge_locations,
                load_card_sprites,
                show_deckbar,
                show_back_btn,
            ),
        )
        .add_systems(
            FixedUpdate,
            (gameover_on_nest_destruction, game_won_on_all_quakka_deaths)
                .run_if(in_state(GameState::InGame).and(not_in_editor)),
        )
        .add_systems(
            OnEnter(IsPaused::False),
            set_message("").run_if(not_in_editor),
        )
        .add_systems(
            OnEnter(LevelProgress::GameOver),
            (pause, set_message("Gameover: nest destroyed")),
        )
        .add_systems(
            OnEnter(LevelProgress::GameWon),
            (pause, set_message("You Win!")),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (
                delete_all::<ArenaBackground>,
                delete_all::<Bridge>,
                delete_all::<LevelEntity>,
                clear_level_memory,
                clear_deckbar,
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
        .init_state::<LevelProgress>()
        .init_resource::<LevelMemory>();
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

fn clear_level_memory(mut memory: ResMut<LevelMemory>) {
    **memory = Level::default();
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

pub struct EnterLevel(pub Level);
impl Command for EnterLevel {
    fn apply(self, world: &mut World) -> () {
        spawn_entities_from_level(&self.0, &mut world.commands());
        world
            .resource_mut::<NextState<GameState>>()
            .set(GameState::InGame);
        world.resource_mut::<LevelMemory>().0 = self.0;
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
    mut level_progress: ResMut<NextState<LevelProgress>>,
    mut card_death_evs: EventReader<CardDeath>,
) {
    for card_death in card_death_evs.read() {
        if Card::Nest == **card_death {
            level_progress.set(LevelProgress::GameOver);
            break;
        }
    }
}

fn game_won_on_all_quakka_deaths(
    quakkas: Query<&Quakka>,
    mut card_death_evs: EventReader<CardDeath>,
    mut level_progress: ResMut<NextState<LevelProgress>>,
) {
    for card_death in card_death_evs.read() {
        if Card::Quakka == **card_death && quakkas.iter().count() == 0 {
            level_progress.set(LevelProgress::GameWon);
            break;
        }
    }
}

fn reset_level_progress(mut level_prog: ResMut<NextState<LevelProgress>>) {
    level_prog.set(LevelProgress::Null);
}
