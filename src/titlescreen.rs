use bevy::prelude::*;
use DuckSlayer::delete_all;

use crate::global::*;

#[derive(Component)]
struct TitleScreen;

#[derive(Component)]
#[require(Button)]
struct PlayBtn;

#[derive(Component)]
#[require(Button)]
struct EditorBtn;

pub fn titlescreen(app: &mut App) {
    app.add_systems(Startup, spawn_titlescreen)
        .add_systems(
            FixedUpdate,
            start_game_on_click.run_if(in_state(GameState::TitleScreen)),
        )
        .add_systems(OnExit(GameState::TitleScreen), delete_all::<TitleScreen>);
}

fn spawn_titlescreen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Node {
        width: Val::Px(BTN_SIZE.0),
        height: Val::Px(BTN_SIZE.1),
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            flex_direction: FlexDirection::Column,
            left: Val::Px(960.),
            top: Val::Px(470.),
            ..default()
        },
        TitleScreen,
        children![
            (
                PlayBtn,
                ImageNode::new(asset_server.load("play-btn.png")),
                button_style.clone()
            ),
            (
                EditorBtn,
                ImageNode::new(asset_server.load("editor-btn.png")),
                button_style.clone()
            )
        ],
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
    interactions: Query<&Interaction, (Changed<Interaction>, With<PlayBtn>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in interactions.iter() {
        if let Interaction::Pressed = interaction {
            game_state.set(GameState::InGame);
        }
    }
}
