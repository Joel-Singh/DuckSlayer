use bevy::{
    color::palettes::{
        css::BLACK,
        tailwind::{BLUE_300, YELLOW_600},
    },
    prelude::*,
};
use DuckSlayer::delete_all;

use crate::{
    back_btn::{hide_back_btn, show_back_btn},
    global::GameState,
    manage_level::{Level, LevelRes},
};

#[derive(Component)]
struct ForCleanup;

#[derive(Resource)]
struct LoadingLevel(Handle<Level>);

enum SelectableLevel {
    One,
    Two,
}

impl SelectableLevel {
    pub fn get_file(&self) -> &str {
        match self {
            SelectableLevel::One => "levels/level_one.level.json",
            SelectableLevel::Two => "levels/level_two.level.json",
        }
    }

    pub fn from_i32(i: i32) -> Option<SelectableLevel> {
        if i == 1 {
            Some(SelectableLevel::One)
        } else if i == 2 {
            Some(SelectableLevel::Two)
        } else {
            None
        }
    }
}

#[derive(Component)]
struct LevelSelectBtn(Option<SelectableLevel>);

pub fn level_select(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::LevelSelect),
        (spawn_title, spawn_level_select_btns, show_back_btn),
    )
    .add_systems(FixedUpdate, (start_loading_level_on_btn_press, load_levels))
    .add_systems(
        OnExit(GameState::LevelSelect),
        (delete_all::<ForCleanup>, hide_back_btn),
    );
}

fn spawn_level_select_btns(mut commands: Commands) {
    let root = commands
        .spawn((
            ForCleanup,
            Node {
                height: Val::Vh(100.),
                width: Val::Vw(100.),
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::fr(10, 1.),
                ..default()
            },
            BackgroundColor(BLUE_300.into()),
            Name::new("Level Select Buttons Container"),
            ZIndex(-1),
        ))
        .id();

    for i in 1..=30 {
        let selectable_level = SelectableLevel::from_i32(i);

        let background_color: Hsla = YELLOW_600.into();
        let muted_background_color =
            background_color.with_lightness(background_color.lightness * 0.5);

        let background_color = if selectable_level.is_some() {
            background_color
        } else {
            muted_background_color
        };

        let level_select_btn = commands
            .spawn((
                Button,
                LevelSelectBtn(selectable_level),
                Node {
                    height: Val::Px(30.),
                    width: Val::Px(30.),
                    margin: UiRect::top(Val::Px(5.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    // To have text centered
                    Text(i.to_string()),
                ],
                BackgroundColor(background_color.into()),
                Name::new("Level Select Button"),
            ))
            .id();

        commands.entity(root).add_child(level_select_btn);
    }
}

fn spawn_title(mut commands: Commands) {
    commands.spawn((
        ForCleanup,
        Text::new("Which level to slay ducks!"),
        TextColor(BLACK.into()),
        Node {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto).with_top(Val::Vh(10.)),
            ..default()
        },
    ));
}

fn start_loading_level_on_btn_press(
    btn_interactions: Query<(&Interaction, &LevelSelectBtn), Changed<Interaction>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (interaction, LevelSelectBtn(level)) in btn_interactions {
        let Some(level) = level else {
            return;
        };

        if *interaction == Interaction::Pressed {
            commands.insert_resource(LoadingLevel(asset_server.load(level.get_file())));
        }
    }
}

fn load_levels(
    loading_level: Option<ResMut<LoadingLevel>>,
    mut commands: Commands,
    mut level_assets: ResMut<Assets<Level>>,
    mut level: ResMut<LevelRes>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let Some(loading_level) = loading_level else {
        return;
    };

    if let Some(loaded_level) = level_assets.remove(loading_level.0.id()) {
        level.0 = loaded_level;
        commands.remove_resource::<LoadingLevel>();
        game_state.set(GameState::InGame);
    }
}
