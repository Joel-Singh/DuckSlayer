use bevy::prelude::*;

use crate::global::GameState;

#[derive(Resource, Deref, DerefMut)]
pub struct InGameUiRoot(Entity);

#[derive(Component)]
#[require(Name::new("InGameUIRoot"))]
struct InGameUiRootMarker;

pub fn ingame_ui_root_plugin(app: &mut App) {
    app.add_systems(PreStartup, spawn_ingame_ui_root)
        .add_systems(OnEnter(GameState::InGame), show)
        .add_systems(OnExit(GameState::InGame), hide)
        .insert_resource(InGameUiRoot(Entity::PLACEHOLDER));
}

fn spawn_ingame_ui_root(mut commands: Commands, mut ingame_ui_root: ResMut<InGameUiRoot>) {
    **ingame_ui_root = commands
        .spawn((
            Node {
                margin: UiRect::left(Val::Auto),
                display: Display::None,
                ..default()
            },
            InGameUiRootMarker,
        ))
        .id()
}

fn hide(mut root: Single<&mut Node, With<InGameUiRootMarker>>) {
    root.display = Display::None;
}

fn show(mut root: Single<&mut Node, With<InGameUiRootMarker>>) {
    root.display = Display::Flex;
}
