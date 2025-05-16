use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    TitleScreen,
    InGame,
}

pub const SCREEN_WIDTH: f32 = 1366.0;
pub const SCREEN_HEIGHT: f32 = 768.0;

pub const DECK_WIDTH: f32 = 0.1 * SCREEN_WIDTH;

pub fn global(app: &mut App) {
    app.init_state::<GameState>();
}
