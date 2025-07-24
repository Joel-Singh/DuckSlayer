use bevy::prelude::*;

use crate::{
    card::Card,
    deckbar::InitializeDeckbar,
    global::GameState,
    ingame_ui_root::InGameUiRoot,
    manage_level::{LevelMemory, WinLoseDeathProgress},
};

#[derive(Component)]
#[require(Name::new("WinInformationRoot"))]
struct WinInformationRoot;

#[derive(Component)]
#[require(Name::new("LoseInformationRoot"))]
struct LoseInformationRoot;

#[derive(Component)]
struct GoalNode;

/// Goal Board is the informational ui that tells the player what they need to keep alive, and what
/// they need to kill
pub fn goal_board_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_root.before(InitializeDeckbar))
        .add_systems(
            FixedUpdate,
            update_board.run_if(in_state(GameState::InGame)),
        );
}

fn spawn_root(
    ingame_ui_root: Res<InGameUiRoot>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let goal_board_root_style = Node {
        margin: UiRect::all(Val::Px(10.0)).with_right(Val::Px(40.0)),
        width: Val::Px(200.),
        column_gap: Val::Px(20.),
        ..default()
    };

    let win_loss_style = Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(10.),
        width: Val::Px(100.),
        ..default()
    };

    let icon_style = Node {
        height: Val::Px(90.),
        ..default()
    };

    commands.entity(**ingame_ui_root).with_child((
        goal_board_root_style.clone(),
        children![
            (
                WinInformationRoot,
                Node {
                    ..win_loss_style.clone()
                },
                children![(
                    icon_style.clone(),
                    ImageNode {
                        image: asset_server.load("skull.png"),
                        ..default()
                    }
                )]
            ),
            (
                LoseInformationRoot,
                Node {
                    margin: UiRect::horizontal(Val::Px(0.0)),
                    ..win_loss_style.clone()
                },
                children![(
                    icon_style.clone(),
                    ImageNode {
                        image: asset_server.load("heart.png"),
                        ..default()
                    }
                )],
            )
        ],
    ));
}

fn update_board(
    goals: Query<Entity, With<GoalNode>>,

    win_information_root: Single<Entity, With<WinInformationRoot>>,
    lose_information_root: Single<Entity, With<LoseInformationRoot>>,

    win_lose_death_progress: Res<WinLoseDeathProgress>,
    level_memory: Res<LevelMemory>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,

    mut prev: Local<Option<(WinLoseDeathProgress, Card, Card)>>,
) {
    let win_card = level_memory.get_win_card();
    let lose_card = level_memory.get_lose_card();

    if prev
        .as_ref()
        .is_some_and(|prev| *prev == (win_lose_death_progress.clone(), *win_card, *lose_card))
    {
        return;
    }

    *prev = Some((win_lose_death_progress.clone(), *win_card, *lose_card));

    for goal in goals {
        commands.entity(goal).despawn();
    }

    let goal_node_style = Node {
        width: Val::Px(50.),
        max_width: Val::Px(100.0),
        max_height: Val::Px(100.0),
        margin: UiRect::horizontal(Val::Auto),
        ..default()
    };

    let mut win_children: Vec<Entity> = Vec::new();
    for _ in 0..win_lose_death_progress.get_win() {
        win_children.push(
            commands
                .spawn((
                    GoalNode,
                    goal_node_style.clone(),
                    ImageNode {
                        image: asset_server.load(win_card.get_sprite_filepath()),
                        ..default()
                    },
                ))
                .id(),
        )
    }

    let mut lose_children: Vec<Entity> = Vec::new();
    for _ in 0..win_lose_death_progress.get_lose() {
        lose_children.push(
            commands
                .spawn((
                    GoalNode,
                    goal_node_style.clone(),
                    ImageNode {
                        image: asset_server.load(lose_card.get_sprite_filepath()),
                        ..default()
                    },
                ))
                .id(),
        )
    }

    commands
        .entity(*win_information_root)
        .insert_children(1, &win_children);

    commands
        .entity(*lose_information_root)
        .insert_children(1, &lose_children);
}
