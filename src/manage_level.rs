use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use std::time::Duration;

use crate::{
    global::{
        GameState, IsPaused, NEST_FIRST_X, NEST_SECOND_X, NEST_Y, QUAKKA_DAMAGE,
        QUAKKA_STARTING_POSITION,
    },
    troops::{spawn_nest, Attacker, Bridge, Health, Nest, Quakka},
};

pub fn manage_level(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        (spawn_entities, spawn_arena_background),
    )
    .add_systems(
        FixedUpdate,
        (
            unpause.run_if(input_just_pressed(KeyCode::Space).and(in_state(GameState::InGame))),
            pause.run_if(nest_destroyed),
        ),
    );
}

fn spawn_arena_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("arena_background.png"),
            ..default()
        },
        Transform {
            // -0.5 so it's in the back and clicks are registered to Nodes
            translation: Vec3::new(0., 0., -0.5),
            ..default()
        },
    ));
}

fn spawn_entities(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Sprite {
            image: asset_server.load("quakka.png"),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform {
            translation: QUAKKA_STARTING_POSITION.extend(0.0),
            ..default()
        },
        Health {
            current_health: 100.0,
            max_health: 100.0,
            healthbar_height: 60.,
        },
        Quakka,
        Attacker {
            cooldown: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
            damage: QUAKKA_DAMAGE,
        },
    ));

    spawn_nest(
        Vec3::new(NEST_FIRST_X, NEST_Y, 0.),
        &mut commands,
        &asset_server,
    );

    spawn_nest(
        Vec3::new(NEST_SECOND_X, NEST_Y, 0.),
        &mut commands,
        &asset_server,
    );

    commands.spawn((
        Bridge,
        Transform {
            translation: Vec3::new(-392.0, -4.0, 0.),
            ..default()
        },
    ));

    commands.spawn((
        Bridge,
        Transform {
            translation: Vec3::new(165.0, -8.0, 0.),
            ..default()
        },
    ));
}

fn unpause(mut is_paused: ResMut<NextState<IsPaused>>) {
    is_paused.set(IsPaused::False);
}

fn pause(mut is_paused: ResMut<NextState<IsPaused>>) {
    is_paused.set(IsPaused::True);
}

fn nest_destroyed(nests: Query<(), With<Nest>>) -> bool {
    nests.iter().count() < 2
}
