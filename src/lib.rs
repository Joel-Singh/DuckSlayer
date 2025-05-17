#![allow(non_snake_case)]

use bevy::prelude::*;

pub fn delete_all<T: Component>(
    entities_to_delete: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &entities_to_delete {
        commands.entity(entity).despawn();
    }
}
