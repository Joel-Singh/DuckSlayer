use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AssetLoad; // Specific Schedule to load assets because OnEnter() runs before Startup

pub fn asset_load_schedule(app: &mut App) {
    app.add_schedule(Schedule::new(AssetLoad));
    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
    main_schedule_order.insert_startup_before(StateTransition, AssetLoad);
}
