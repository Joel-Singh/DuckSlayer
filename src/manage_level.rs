use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use game_messages::set_message;

use std::time::Duration;

use crate::{
    global::{
        GameState, NEST_FIRST_X, NEST_SECOND_X, NEST_Y, QUAKKA_DAMAGE, QUAKKA_STARTING_POSITION,
    },
    troops::{spawn_nest, Attacker, Bridge, Health, Nest, Quakka},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum IsPaused {
    True,
    False,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States, Default)]
pub enum GameOver {
    True,
    #[default]
    False,
}

pub fn manage_level(app: &mut App) {
    app.add_plugins(game_messages::game_messages)
        .add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_entities,
                spawn_arena_background,
                set_message("[Space] to start level"),
            ),
        )
        .add_systems(OnEnter(IsPaused::False), set_message(""))
        .add_systems(
            FixedUpdate,
            (
                unpause.run_if(input_just_pressed(KeyCode::Space).and(run_once)),
                set_gameover_true.run_if(nest_destroyed),
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            OnEnter(GameOver::True),
            (pause, set_message("Gameover: nest destroyed")),
        )
        .insert_state::<IsPaused>(IsPaused::True)
        .init_state::<GameOver>();
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

fn set_gameover_true(mut gameover: ResMut<NextState<GameOver>>) {
    gameover.set(GameOver::True);
}

fn nest_destroyed(nests: Query<(), With<Nest>>) -> bool {
    nests.iter().count() < 2
}

mod game_messages {
    use bevy::{
        ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
        prelude::*,
    };

    #[derive(Component)]
    struct MessageBox;

    pub fn game_messages(app: &mut App) {
        app.add_systems(Startup, spawn_message_box);
    }

    fn spawn_message_box(mut commands: Commands) {
        commands.spawn((
            Text::new(""),
            Node {
                margin: UiRect::all(Val::Auto).with_top(Val::Px(80.)),
                ..default()
            },
            BoxShadow::new(
                Color::BLACK.with_alpha(0.8),
                Val::ZERO,
                Val::ZERO,
                Val::Percent(0.),
                Val::Px(10.),
            ),
            MessageBox,
        ));
    }

    pub fn set_message(message: &'static str) -> ScheduleConfigs<ScheduleSystem> {
        return (move |mut commands: Commands, message_box: Single<Entity, With<MessageBox>>| {
            commands.entity(*message_box).insert(Text::new(message));
        })
        .into_configs();
    }
}
