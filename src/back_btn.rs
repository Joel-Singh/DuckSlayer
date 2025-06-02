use crate::global::GameState;
use bevy::prelude::*;
use bevy_egui::input::egui_wants_any_pointer_input;

#[derive(Component)]
pub struct BackBtn;

pub fn back_btn(app: &mut App) {
    app.add_systems(Startup, spawn_back_btn).add_systems(
        FixedUpdate,
        go_to_titlescreen_on_click.run_if(not(egui_wants_any_pointer_input)),
    );
}

fn spawn_back_btn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        BackBtn,
        Node {
            margin: UiRect::all(Val::Px(30.)),
            display: Display::None,
            ..default()
        },
        Button,
        ImageNode::new(asset_server.load("back-btn.png")),
    ));
}

fn go_to_titlescreen_on_click(
    interactions: Query<&Interaction, (Changed<Interaction>, With<BackBtn>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in interactions {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::TitleScreen);
        }
    }
}

pub fn show_back_btn(mut back_btn: Single<&mut Node, With<BackBtn>>) {
    back_btn.display = Display::Flex;
}

pub fn hide_back_btn(mut back_btn: Single<&mut Node, With<BackBtn>>) {
    back_btn.display = Display::None;
}
