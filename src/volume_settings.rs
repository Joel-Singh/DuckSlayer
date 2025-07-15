use bevy::{audio::Volume, prelude::*};

#[derive(Resource)]
pub struct VolumeSettings {
    sfx_vol: Volume, // from 0.0 to 1.0
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
        self.sfx_mute = x;
    }
}

pub fn volume_settings_plugin(app: &mut App) {
    app.insert_resource(VolumeSettings {
        sfx_vol: Volume::default(),
        sfx_mute: false,
        music_vol: Volume::default(),
        music_mute: false,
    });
}
