use crate::global::{in_editor, GameState};
use bevy::color::palettes::tailwind::LIME_400;
use bevy::color::palettes::tailwind::ORANGE_400;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct SaveIndicator;

const SAVED_COLOR: Srgba = LIME_400;
const NOT_SAVED_COLOR: Srgba = ORANGE_400;
const SIZE: f32 = 20.;

pub fn save_indicator_plugin(app: &mut App) {
    app.add_systems(Startup, init)
        .add_systems(OnEnter(GameState::InGame), show.run_if(in_editor))
        .add_systems(
            FixedUpdate,
            follow_cursor.run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(GameState::InGame), hide);
}

pub fn set_saved(save_indicator: Single<Entity, With<SaveIndicator>>, mut commands: Commands) {
    commands
        .entity(*save_indicator)
        .insert(BackgroundColor(SAVED_COLOR.into()));
}

pub fn set_not_saved(save_indicator: Single<Entity, With<SaveIndicator>>, mut commands: Commands) {
    commands
        .entity(*save_indicator)
        .insert(BackgroundColor(NOT_SAVED_COLOR.into()));
}

fn init(mut commands: Commands) {
    commands.spawn((
        SaveIndicator,
        Node {
            position_type: PositionType::Absolute,
            display: Display::None,
            width: Val::Px(SIZE),
            height: Val::Px(SIZE),
            ..default()
        },
        Pickable {
            should_block_lower: false,
            is_hoverable: false,
        },
        BackgroundColor(SAVED_COLOR.into()),
        BorderRadius::MAX,
    ));
}

fn show(mut save_indicator: Single<&mut Node, With<SaveIndicator>>) {
    save_indicator.display = Display::DEFAULT;
}

fn hide(mut save_indicator: Single<&mut Node, With<SaveIndicator>>) {
    save_indicator.display = Display::None;
}

fn follow_cursor(
    mut save_indicator: Single<&mut Node, With<SaveIndicator>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if let Some(cursor_position) = window.cursor_position() {
        let mut left = cursor_position.x;
        let mut top = cursor_position.y;

        // center
        left -= SIZE / 2.;
        top -= SIZE / 2.;

        save_indicator.left = Val::Px(left);
        save_indicator.top = Val::Px(top);
    }
}
