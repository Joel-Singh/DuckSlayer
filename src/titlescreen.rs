use bevy::prelude::*;

use crate::global::*;
use crate::util;

#[derive(Component)]
struct TitleScreen;

pub fn title_screen(app: &mut App) {
    app.add_systems(Startup, spawn_titlescreen)
        .add_systems(
            FixedUpdate,
            start_game_on_click.run_if(in_state(GameState::TitleScreen)),
        )
        .add_systems(
            OnExit(GameState::TitleScreen),
            util::delete_all::<TitleScreen>,
        );
}

fn spawn_titlescreen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            ..default()
        },
        Button,
        TitleScreen,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("title_screen.png"),
            ..default()
        },
        Transform {
            // -0.5 so it's in the back and clicks are registered to Nodes
            translation: Vec3::new(0., 0., -0.5),
            ..default()
        },
        TitleScreen,
    ));
}

fn start_game_on_click(
    interactions: Query<&Interaction, Changed<Interaction>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in interactions.iter() {
        if let Interaction::Pressed = interaction {
            game_state.set(GameState::InGame);
        }
    }
}
