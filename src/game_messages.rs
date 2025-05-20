use bevy::{
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    prelude::*,
};

use crate::global::{GameState, IsPaused};

#[derive(Component)]
struct MessageBox;

pub fn game_messages(app: &mut App) {
    app.add_systems(Startup, spawn_message_box)
        .add_systems(
            OnEnter(GameState::InGame),
            set_message("[Space] to start level".to_string()),
        )
        .add_systems(OnEnter(IsPaused::False), set_message("".to_string()));
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

fn set_message(message: String) -> ScheduleConfigs<ScheduleSystem> {
    return (move |mut commands: Commands, message_box: Single<Entity, With<MessageBox>>| {
        commands
            .entity(*message_box)
            .insert(Text::new(message.clone()));
    })
    .into_configs();
}
