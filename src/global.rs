use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::input::EguiWantsInput;
use std::env;

use crate::asset_load_schedule::AssetLoad;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    TitleScreen,
    InGame,
    LevelSelect,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct InEditorRes(bool);

pub fn in_editor(in_editor_res: Res<InEditorRes>) -> bool {
    **in_editor_res
}

pub fn not_in_editor(in_editor_res: Res<InEditorRes>) -> bool {
    !in_editor(in_editor_res)
}

#[derive(Resource, PartialEq)]
pub struct IsDebug(pub bool);

impl Default for IsDebug {
    fn default() -> Self {
        if let Ok(duckslayer_debug) = env::var("DUCKSLAYER_DEBUG") {
            if duckslayer_debug == "true" {
                return IsDebug(true);
            } else {
                return IsDebug(false);
            }
        }
        IsDebug(false)
    }
}

pub fn in_debug(is_debug: Res<IsDebug>) -> bool {
    is_debug.0
}

#[derive(Resource, Default)]
pub struct ImageHandles {
    pub play_btn: Handle<Image>,
    pub editor_btn: Handle<Image>,
    pub titlescreen_background: Handle<Image>,
    pub arena_background: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct CursorWorldCoords(pub Vec2);

#[derive(Resource, Default, Debug)]
pub struct IsPointerOverUi(pub bool);

pub const SCREEN_WIDTH: f32 = 1366.0;

pub const DECK_WIDTH: f32 = 0.1 * SCREEN_WIDTH;

pub const FARMER_SPEED: f32 = 25.0;

pub const NEST_POSITIONS: ((f32, f32), (f32, f32)) = ((-388., -312.), (173., -312.));

pub const BRIDGE_LOCATIONS: (Vec2, Vec2) = (Vec2::new(-392.0, -4.0), Vec2::new(165.0, -8.0));

// LHS is image size
pub const BTN_SIZE: (f32, f32) = (360. / 3.0, 160. / 3.0);

pub fn global(app: &mut App) {
    app.add_systems(AssetLoad, load_images)
        .add_systems(
            FixedPreUpdate,
            (update_cursor_world_coords, update_is_pointer_over_ui),
        )
        .init_resource::<CursorWorldCoords>()
        .init_resource::<IsDebug>()
        .init_resource::<IsPointerOverUi>()
        .init_resource::<InEditorRes>()
        .init_state::<GameState>();
}

fn update_cursor_world_coords(
    windows_q: Single<&Window, With<PrimaryWindow>>,
    camera_q: Single<(&GlobalTransform, &Camera)>,
    mut cursor_world_coords_res: ResMut<CursorWorldCoords>,
) {
    let cursor_window_position = windows_q.cursor_position();
    if let Some(cursor_window_position) = cursor_window_position {
        let (camera_transform, camera) = *camera_q;
        if let Ok(cursor_world_coords) =
            camera.viewport_to_world_2d(camera_transform, cursor_window_position)
        {
            cursor_world_coords_res.0 = cursor_world_coords;
        }
    }
}

fn update_is_pointer_over_ui(
    mut is_pointer_over_ui: ResMut<IsPointerOverUi>,
    interaction_query: Query<&Interaction, With<Node>>,

    egui_wants_input: Res<EguiWantsInput>,
) {
    let bevy_using_pointer = interaction_query.iter().any(|i| *i != Interaction::None);

    is_pointer_over_ui.0 = bevy_using_pointer || egui_wants_input.wants_any_pointer_input();
}

fn load_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ImageHandles {
        play_btn: asset_server.load("play-btn.png"),
        editor_btn: asset_server.load("editor-btn.png"),
        titlescreen_background: asset_server.load("titlescreen.png"),
        arena_background: asset_server.load("arena-background.png"),
    })
}
