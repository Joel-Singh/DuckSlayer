use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Serialize, Deserialize, Asset, TypePath)]
pub struct CardConsts {
    pub farmer: FarmerConsts,
    pub quakka: QuakkaConsts,
    pub waterball: WaterballConsts,
    pub nest: NestConsts,
}

pub fn card_constants(app: &mut App) {
    app.init_resource::<CardConsts>();
}

#[derive(Serialize, Deserialize)]
pub struct WaterballConsts {
    pub radius: f32,
    pub damage: f32,
}

impl Default for WaterballConsts {
    fn default() -> WaterballConsts {
        const RADIUS: f32 = 50.;
        WaterballConsts {
            radius: RADIUS,
            damage: 30.,
        }
    }
}

impl WaterballConsts {
    pub fn size(&self) -> (f32, f32) {
        (self.radius * 2., self.radius * 2.)
    }
}

#[derive(Serialize, Deserialize)]
pub struct FarmerConsts {
    pub size: (f32, f32),
}

impl Default for FarmerConsts {
    fn default() -> Self {
        const FILE_DIMENSIONS: (f32, f32) = (1399., 1238.);
        const CUSTOM_WIDTH: f32 = 60.;
        const SIZE: (f32, f32) = (
            FILE_DIMENSIONS.0 * (CUSTOM_WIDTH / FILE_DIMENSIONS.0),
            FILE_DIMENSIONS.1 * (CUSTOM_WIDTH / FILE_DIMENSIONS.0),
        );

        FarmerConsts { size: SIZE }
    }
}

#[derive(Serialize, Deserialize)]
pub struct QuakkaConsts {
    pub size: (f32, f32),
    pub damage: f32,
    pub hit_distance: f32,
    pub speed: f32,
}

impl Default for QuakkaConsts {
    fn default() -> Self {
        QuakkaConsts {
            size: (100., 100.),
            damage: 60.0,
            hit_distance: 50.0,
            speed: 75.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NestConsts {
    pub size: (f32, f32),
}

impl Default for NestConsts {
    fn default() -> Self {
        NestConsts { size: (50., 50.) }
    }
}
