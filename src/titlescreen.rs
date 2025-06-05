use bevy::prelude::*;
use DuckSlayer::delete_all;

use crate::{asset_load_schedule::AssetLoad, global::*};

#[derive(Component)]
struct TitleScreen;

#[derive(Component)]
#[require(Button)]
struct PlayBtn;

#[derive(Component)]
#[require(Button)]
struct EditorBtn;

#[derive(Resource, Default)]
struct ImageHandles {
    play_btn: Handle<Image>,
    editor_btn: Handle<Image>,
    background: Handle<Image>,
}

pub fn titlescreen(app: &mut App) {
    app.add_systems(AssetLoad, load_images)
        .add_systems(OnEnter(GameState::TitleScreen), spawn_titlescreen)
        .add_systems(
            FixedUpdate,
            (start_game_on_click, start_editor_on_click).run_if(in_state(GameState::TitleScreen)),
        )
        .add_systems(OnExit(GameState::TitleScreen), delete_all::<TitleScreen>);
}

fn load_images(world: &mut World) {
    world.insert_resource(ImageHandles {
        play_btn: world.load_asset("play-btn.png"),
        editor_btn: world.load_asset("editor-btn.png"),
        background: world.load_asset("titlescreen.png"),
    })
}

fn spawn_titlescreen(mut commands: Commands, handles: Res<ImageHandles>) {
    let button_style = Node {
        width: Val::Px(BTN_SIZE.0),
        height: Val::Px(BTN_SIZE.1),
        ..default()
    };

    commands.spawn((
        Sprite {
            image: handles.background.clone(),
            ..default()
        },
        Transform {
            // -0.5 so it's in the back and clicks are registered to Nodes
            translation: Vec3::new(0., 0., -0.5),
            ..default()
        },
        TitleScreen,
    ));

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
                ImageNode::new(handles.play_btn.clone()),
                button_style.clone()
            ),
            (
                EditorBtn,
                ImageNode::new(handles.editor_btn.clone()),
                button_style.clone()
            )
        ],
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

fn start_editor_on_click(
    interactions: Query<&Interaction, (Changed<Interaction>, With<EditorBtn>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut is_in_editor: ResMut<NextState<IsInEditor>>,
) {
    for interaction in interactions.iter() {
        if let Interaction::Pressed = interaction {
            game_state.set(GameState::InGame);
            is_in_editor.set(IsInEditor::True);
        }
    }
}
