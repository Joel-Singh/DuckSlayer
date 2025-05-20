use crate::global::*;
use crate::nest::spawn_nest;
use crate::troops::*;
use bevy::prelude::*;

use std::time::Duration;

pub fn initialize_game(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        (spawn_entities, spawn_arena_background),
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
            translation: Vec3::new(0., 200., 0.),
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
