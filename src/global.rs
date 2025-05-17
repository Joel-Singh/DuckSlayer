use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    TitleScreen,
    InGame,
}

#[derive(Resource, PartialEq)]
pub struct IsPaused(pub bool);

impl Default for IsPaused {
    fn default() -> Self {
        return IsPaused(true);
    }
}

#[derive(Resource, Default)]
pub struct CursorWorldCoords(pub Vec2);

pub const SCREEN_WIDTH: f32 = 1366.0;
pub const SCREEN_HEIGHT: f32 = 768.0;

pub const DECK_WIDTH: f32 = 0.1 * SCREEN_WIDTH;

pub const FARMER_SPRITE_SIZE: (f32, f32) = (1399., 1238.);
pub const FARMER_CUSTOM_WIDTH: f32 = 60.;
pub const FARMER_SIZE: Vec2 = Vec2::new(
    FARMER_SPRITE_SIZE.0 * (FARMER_CUSTOM_WIDTH / FARMER_SPRITE_SIZE.0),
    FARMER_SPRITE_SIZE.1 * (FARMER_CUSTOM_WIDTH / FARMER_SPRITE_SIZE.0),
);

pub fn global(app: &mut App) {
    app.add_systems(FixedUpdate, update_cursor_world_coords)
        .init_resource::<CursorWorldCoords>()
        .init_resource::<IsPaused>()
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
