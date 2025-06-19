use bevy::{
    ecs::system::RunSystemOnce,
    prelude::*,
    tasks::{futures_lite::future, AsyncComputeTaskPool, Task},
};
use rfd::FileDialog;
use std::path::PathBuf;
use DuckSlayer::delete_all;

use crate::{
    card::CardConsts,
    deckbar::clear_deckbar,
    manage_level::{spawn_entities_from_level_memory, Level, LevelEntity, LevelMemory},
};

pub fn saving_loading_levels_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, poll_filepicker_completion);
}

#[derive(Component)]
struct PickingFile(Task<Option<PathBuf>>);

#[derive(Event)]
struct FinishedPickingFile(PathBuf);

pub struct SaveLevelWithFileDialog;
impl Command for SaveLevelWithFileDialog {
    fn apply(self, world: &mut World) {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            FileDialog::new()
                .set_file_name("placeholder.level.json")
                .add_filter("JSON", &["level.json"])
                .save_file()
        });

        world.spawn(PickingFile(task)).observe(
            |trigger: Trigger<FinishedPickingFile>, world: &mut World| {
                let level = Level::get_current(world);
                let picked_file = &trigger.0;

                let result =
                    std::fs::write(picked_file, serde_json::to_string_pretty(&level).unwrap());

                if let Err(_) = result {
                    warn!("Something has gone wrong saving the level");
                };

                world.entity_mut(trigger.target()).despawn();
            },
        );
    }
}

fn poll_filepicker_completion(
    mut tasks: Query<(Entity, &mut PickingFile)>,
    mut commands: Commands,
) {
    for (e, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            if let Some(file) = result {
                commands.entity(e).trigger(FinishedPickingFile(file));
            } else {
                commands.entity(e).despawn();
            };
        }
    }
}

pub struct LoadLevelWithFileDialog;
impl Command for LoadLevelWithFileDialog {
    fn apply(self, world: &mut World) {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            FileDialog::new()
                .add_filter("JSON", &["level.json"])
                .pick_file()
        });

        world.spawn(PickingFile(task)).observe(
            |trigger: Trigger<FinishedPickingFile>, world: &mut World| {
                let mut level_res = world.resource_mut::<LevelMemory>();
                let file = &trigger.0;

                if let Ok(file) = std::fs::read(file) {
                    let level_from_file =
                        serde_json::from_str::<Level>(&String::from_utf8(file).unwrap());

                    if let Ok(level_from_file) = level_from_file {
                        level_res.0 = level_from_file;
                        let _ = world.run_system_once(delete_all::<LevelEntity>);
                        let _ = world.run_system_once(clear_deckbar);
                        let _ = world.run_system_once(spawn_entities_from_level_memory);
                    } else {
                        warn!("Couldn't load level from file");
                    }
                } else {
                    warn!("Couldn't read file when loading level");
                }

                world.entity_mut(trigger.target()).despawn();
            },
        );
    }
}

pub struct SaveCardConstsWithFileDialog;
impl Command for SaveCardConstsWithFileDialog {
    fn apply(self, world: &mut World) {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            FileDialog::new()
                .set_file_name("placeholder.consts.json")
                .add_filter("JSON", &["consts.json"])
                .save_file()
        });

        world.spawn(PickingFile(task)).observe(
            |trigger: Trigger<FinishedPickingFile>, world: &mut World| {
                let card_consts = world.resource::<CardConsts>();
                let picked_file = &trigger.0;

                let result = std::fs::write(
                    picked_file,
                    serde_json::to_string_pretty(&card_consts).unwrap(),
                );

                if let Err(_) = result {
                    warn!("Something has gone wrong saving the level");
                };

                world.entity_mut(trigger.target()).despawn();
            },
        );
    }
}

pub struct LoadCardConstsWithFileDialog;
impl Command for LoadCardConstsWithFileDialog {
    fn apply(self, world: &mut World) {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            FileDialog::new()
                .add_filter("JSON", &["consts.json"])
                .pick_file()
        });

        world.spawn(PickingFile(task)).observe(
            |trigger: Trigger<FinishedPickingFile>, world: &mut World| {
                let mut card_consts = world.resource_mut::<CardConsts>();
                let file = &trigger.0;

                if let Ok(file) = std::fs::read(file) {
                    let card_consts_from_file =
                        serde_json::from_str::<CardConsts>(&String::from_utf8(file).unwrap());

                    if let Ok(card_consts_from_file) = card_consts_from_file {
                        *card_consts = card_consts_from_file;
                    } else {
                        warn!("Couldn't load level from file");
                    }
                } else {
                    warn!("Couldn't read file when loading level");
                }

                world.entity_mut(trigger.target()).despawn();
            },
        );
    }
}
