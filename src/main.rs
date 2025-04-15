use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            //level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_systems(Startup, (setup_camera, spawn_quacka))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_quacka(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(( Sprite {
        image: asset_server.load("quacka.png"),
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
    }, Transform {
            translation: Vec3::new(0., 200., 0.),
            ..default()
        } ));
}
