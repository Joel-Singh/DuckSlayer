use bevy::prelude::*;

#[derive(Component)]
struct Quacka;

#[derive(Component)]
struct Nest;

const QUACKA_SPEED: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            //level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_systems(Startup, (setup_camera, spawn_quacka))
        .add_systems(FixedUpdate, quacka_go_to_nest)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_quacka(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("quacka.png"),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 200., 0.),
            ..default()
        },
        Quacka,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("nest.png"),
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 0., 0.),
            ..default()
        },
        Nest,
    ));
}

fn quacka_go_to_nest(
    mut quackas: Query<&mut Transform, (With<Quacka>, Without<Nest>)>,
    nest: Single<&Transform, (With<Nest>, Without<Quacka>)>,
    time: Res<Time>,
) {
    for mut quacka in quackas.iter_mut() {
        let mut difference = nest.translation - quacka.translation;
        difference = difference.normalize();

        quacka.translation = quacka.translation + (difference) * time.delta_secs() * QUACKA_SPEED;
    }
}
