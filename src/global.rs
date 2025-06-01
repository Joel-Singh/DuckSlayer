use bevy::{prelude::*, window::PrimaryWindow};
use std::env;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    TitleScreen,
    InGame,
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
pub struct CursorWorldCoords(pub Vec2);

pub const SCREEN_WIDTH: f32 = 1366.0;

pub const DECK_WIDTH: f32 = 0.1 * SCREEN_WIDTH;

pub const FARMER_SPRITE_SIZE: (f32, f32) = (1399., 1238.);
pub const FARMER_CUSTOM_WIDTH: f32 = 60.;
pub const FARMER_SIZE: Vec2 = Vec2::new(
    FARMER_SPRITE_SIZE.0 * (FARMER_CUSTOM_WIDTH / FARMER_SPRITE_SIZE.0),
    FARMER_SPRITE_SIZE.1 * (FARMER_CUSTOM_WIDTH / FARMER_SPRITE_SIZE.0),
);
pub const FARMER_SPEED: f32 = 25.0;

pub const QUAKKA_SPEED: f32 = 75.0;
pub const QUAKKA_HIT_DISTANCE: f32 = 50.0;
pub const QUAKKA_DAMAGE: f32 = 60.0;
pub const QUAKKA_SIZE: Vec2 = Vec2::new(100.0, 100.0);

pub const WATERBALL_RADIUS: f32 = 50.;
pub const WATERBALL_SIZE: Vec2 = Vec2::new(WATERBALL_RADIUS * 2., WATERBALL_RADIUS * 2.);
pub const WATERBALL_DAMAGE: f32 = 30.0;

pub const NEST_FIRST_X: f32 = -388.;
pub const NEST_SECOND_X: f32 = 173.;
pub const NEST_Y: f32 = -312.;
pub const NEST_ATTACK_DISTANCE: f32 = 500.;
pub const NEST_DAMAGE: f32 = 10.;

pub const QUAKKA_STARTING_POSITION: Vec2 = Vec2::new(-391., 104.);

pub fn global(app: &mut App) {
    app.add_systems(FixedUpdate, update_cursor_world_coords)
        .init_resource::<CursorWorldCoords>()
        .init_resource::<IsDebug>()
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
