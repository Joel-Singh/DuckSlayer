use bevy::prelude::*;
use DuckSlayer::delete_all;
use crate::settings_screen::ShowSettingsScreen;

use crate::{
    global::*,
    manage_level::{EnterLevel, Level},
};

#[derive(Component)]
#[require(Name::new("Titlescreen"))]
struct TitleScreen;

#[derive(Component)]
#[require(Button, Name::new("Play Btn"))]
struct PlayBtn;

#[derive(Component)]
#[require(Button, Name::new("Editor Btn"))]
struct EditorBtn;

#[derive(Component)]
#[require(Button, Name::new("Settings Btn"))]
struct SettingsBtn;

pub fn titlescreen(app: &mut App) {
    app.add_systems(OnEnter(GameState::TitleScreen), spawn_titlescreen)
        .add_systems(
            FixedUpdate,
            (start_game_on_click, start_editor_on_click, show_settings_on_click).run_if(in_state(GameState::TitleScreen)),
        )
        .add_systems(OnExit(GameState::TitleScreen), delete_all::<TitleScreen>);
}

fn spawn_titlescreen(mut commands: Commands, handles: Res<ImageHandles>, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: handles.titlescreen_background.clone(),
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
            row_gap: Val::Px(15.),
            left: Val::Px(960.),
            top: Val::Px(470.),
            ..default()
        },
        TitleScreen,
        children![
            (
                PlayBtn,
                btn_bundle(),
                btn_style(),
                children![(
                    Text::new("PLAY"),
                    text_bundle(&asset_server),
                )]
            ),
            (
                EditorBtn,
                btn_bundle(),
                btn_style(),
                children![(
                    Text::new("EDITOR"),
                    text_bundle(&asset_server),
                )]
            ),
            (
                SettingsBtn,
                btn_bundle(),
                btn_style(),
                children![(
                    Text::new("SETTINGS"),
                    text_bundle(&asset_server),
                )]
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
            game_state.set(GameState::LevelSelect);
        }
    }
}

fn start_editor_on_click(
    interactions: Query<&Interaction, (Changed<Interaction>, With<EditorBtn>)>,
    mut in_editor: ResMut<InEditorRes>,

    mut commands: Commands,
) {
    for interaction in interactions.iter() {
        if let Interaction::Pressed = interaction {
            **in_editor = true;
            commands.queue(EnterLevel(Level::get_stub()));
        }
    }
}

fn show_settings_on_click(
    interactions: Query<&Interaction, (Changed<Interaction>, With<SettingsBtn>)>,
    mut commands: Commands,
) {
    for interaction in interactions.iter() {
        if let Interaction::Pressed = interaction {
            commands.queue(ShowSettingsScreen);
        }
    }
}


fn btn_bundle() -> impl Bundle {
    let background_color: Color = Srgba::hex("#ffd966ff").unwrap().into();
    (
        BackgroundColor(background_color),
        BorderColor(Color::BLACK),
        BorderRadius::all(Val::Px(25.)),
    )
}

fn btn_style() -> Node {
    Node {
        border: UiRect::all(Val::Px(5.)),
        width: Val::Px(BTN_SIZE.0),
        height: Val::Px(BTN_SIZE.1),
        ..default()
    }
}

fn text_bundle(asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        TextFont {
            font: asset_server.load("DynaPuff-Regular.ttf"),
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            margin: UiRect::all(Val::Auto),
            ..default()
        }
    )
}
