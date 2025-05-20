use std::time::Duration;

use bevy::prelude::*;

use crate::{
    global::{NEST_ATTACK_DISTANCE, NEST_DAMAGE},
    troops::{Attacker, Chaseable, Farmer, Health},
};

#[derive(Component)]
#[require(Chaseable)]
struct Nest;

pub fn nest(app: &mut App) {
    app.add_systems(FixedUpdate, nest_shoot);
}

pub fn spawn_nest(translation: Vec3, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("nest.png"),
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        Transform {
            translation,
            ..default()
        },
        Health {
            current_health: 100.0,
            max_health: 100.0,
            healthbar_height: 60.,
        },
        Nest,
        Attacker {
            cooldown: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
            damage: NEST_DAMAGE,
        },
    ));
}

fn nest_shoot(
    mut victims: Query<(&Transform, &mut Health), (Without<Nest>, Without<Farmer>)>,
    mut nests: Query<(&Transform, &mut Attacker), With<Nest>>,
) {
    for mut nest in nests {
        let closest_victim = victims.iter_mut().min_by(|a, b| {
            let a_dist = nest.0.translation.distance(a.0.translation);
            let b_dist = nest.0.translation.distance(b.0.translation);
            a_dist.total_cmp(&b_dist)
        });

        if closest_victim.is_none() {
            continue;
        }

        let mut closest_victim = closest_victim.unwrap();

        let dist_to_victim = nest.0.translation.distance(closest_victim.0.translation);

        if dist_to_victim < NEST_ATTACK_DISTANCE && nest.1.cooldown.finished() {
            nest.1.cooldown.reset();
            closest_victim.1.current_health -= NEST_DAMAGE;
        }
    }
}
