use bevy::{
    color::palettes::{
        css::BLACK,
        tailwind::{BLUE_300, GRAY_950, YELLOW_600},
    },
    prelude::*,
    state::commands,
};
use DuckSlayer::delete_all;

use crate::global::GameState;

#[derive(Component)]
struct ForCleanup;

#[derive(Component)]
struct LevelSelectBtn(i32);

#[derive(Component)]
struct HasImplementedLevel;

pub fn level_select(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::LevelSelect),
        (spawn_title, spawn_level_select_btns),
    )
    .add_systems(FixedUpdate, enter_level_on_btn_press)
    .add_systems(OnExit(GameState::LevelSelect), delete_all::<ForCleanup>);
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
        ))
        .id();

    for i in 1..=30 {
        let background_color: Hsla = YELLOW_600.into();
        let muted_background_color =
            background_color.with_lightness(background_color.lightness * 0.5);

        let mut level_select_btn = commands.spawn((
            Button,
            LevelSelectBtn(i),
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
            BackgroundColor(muted_background_color.into()),
            Name::new("Level Select Button"),
        ));

        if (i == 1) || (i == 2) {
            level_select_btn.insert(HasImplementedLevel);
            level_select_btn.insert(BackgroundColor(background_color.into()));
        }

        let level_select_btn = level_select_btn.id();

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
        ZIndex(1),
    ));
}

fn enter_level_on_btn_press(
    btn_interactions: Query<
        (&Interaction, &LevelSelectBtn),
        (With<HasImplementedLevel>, Changed<Interaction>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, LevelSelectBtn(level)) in btn_interactions {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::InGame);
        }
    }
}
