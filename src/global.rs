use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    TitleScreen,
    InGame,
}

pub fn global(app: &mut App) {
    app.init_state::<GameState>();
}
