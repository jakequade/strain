use amethyst::{
    core::transform::Transform,
    ecs::{prelude::Component, storage::DenseVecStorage},
    prelude::*,
    renderer::{Camera, SpriteRender},
};

use crate::{
    states::LoadState,
    asset_loader::{load_sprite_sheet, SpriteSheetHandle},
    components::{init_dude, Dude},
};

pub const DUDE_HEIGHT_WIDTH: f32 = 32.0;
pub const WINDOW_HEIGHT: f32 = 500.0;
pub const WINDOW_WIDTH: f32 = 1000.0;

//#region Camera

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WINDOW_WIDTH * 0.5, WINDOW_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with(transform)
        .build();
}

//#endregion

//#region Strain

/// The game struct.
/// - `sprite_sheet` needs to be an `Option` so that `Strain::default()` can be invoked.
#[derive(Default)]
pub struct Strain {
    sprite_sheet: Option<SpriteSheetHandle>,
}

impl SimpleState for Strain {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.sprite_sheet.replace(load_sprite_sheet(world));

        init_camera(world);
        init_dude(world, self.sprite_sheet.clone().unwrap());
    }
}

//#endregion

//#region floor

// TODO: Finish this region once we have a moveable character

pub struct Floor {}

impl Floor {
    pub fn new() -> Floor {
        Floor {}
    }
}

impl Component for Floor {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_floor(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    let mut floor_transform = Transform::default();
    floor_transform.set_translation_xyz(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Floor::new())
        .with(floor_transform)
        .build();
}

//#endregion
