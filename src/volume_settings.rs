use crate::debug::in_debug;
use bevy::{audio::Volume, prelude::*};

#[derive(Resource)]
pub struct VolumeSettings {
    sfx_vol: Volume,
    sfx_mute: bool,
    music_vol: Volume,
    music_mute: bool,
}

impl VolumeSettings {
    pub fn get_sfx(&self) -> Volume {
        if self.sfx_mute {
            Volume::SILENT
        } else {
            self.sfx_vol
        }
    }

    pub fn get_music(&self) -> Volume {
        if self.music_mute {
            Volume::SILENT
        } else {
            self.music_vol
        }
    }

    pub fn set_sfx_vol(&mut self, x: Volume) {
        self.sfx_vol = x;
    }

    pub fn set_music_vol(&mut self, x: Volume) {
        self.music_vol = x;
    }

    pub fn set_sfx_mute(&mut self, x: bool) {
        self.sfx_mute = x;
    }

    pub fn set_music_mute(&mut self, x: bool) {
        self.music_mute = x;
    }

    pub fn get_sfx_mute(&self) -> bool {
        self.sfx_mute
    }

    pub fn get_music_mute(&self) -> bool {
        self.music_mute
    }
}

pub fn volume_settings_plugin(app: &mut App) {
    app.insert_resource(VolumeSettings {
        sfx_vol: Volume::Linear(0.5),
        sfx_mute: false,
        music_vol: Volume::Linear(0.5),
        music_mute: false,
    });

    if in_debug() {
        app.add_systems(FixedUpdate, debug_display);
    }
}

use crate::debug_ui::DisplayInDebug;

pub fn debug_display(
    volume_settings: Res<VolumeSettings>,
    mut display_in_debug: ResMut<DisplayInDebug>,
) {
    display_in_debug.insert(
        "Music Volume".to_string(),
        volume_settings.get_music().to_linear().to_string(),
    );

    display_in_debug.insert(
        "SFX Volume".to_string(),
        volume_settings.get_sfx().to_linear().to_string(),
    );
}
