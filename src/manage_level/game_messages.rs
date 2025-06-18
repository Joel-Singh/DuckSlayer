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
        Pickable {
            should_block_lower: false,
            ..default()
        },
    ));
}

pub fn set_message(message: &'static str) -> ScheduleConfigs<ScheduleSystem> {
    return (move |mut commands: Commands, message_box: Single<Entity, With<MessageBox>>| {
        commands.entity(*message_box).insert(Text::new(message));
    })
    .into_configs();
}
