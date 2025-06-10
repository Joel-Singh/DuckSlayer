use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CardConsts {
    pub farmer: FarmerConsts,
    pub quakka: QuakkaConsts,
    pub waterball: WaterballConsts,
    pub nest: NestConsts,
}

pub fn card_constants(app: &mut App) {
    app.init_resource::<CardConsts>();
}

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

pub struct QuakkaConsts {
    pub size: (f32, f32),
}

impl Default for QuakkaConsts {
    fn default() -> Self {
        QuakkaConsts { size: (100., 100.) }
    }
}

pub struct NestConsts {
    pub size: (f32, f32),
}

impl Default for NestConsts {
    fn default() -> Self {
        NestConsts { size: (50., 50.) }
    }
}
