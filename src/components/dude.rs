use amethyst::{
    core::transform::Transform,
    ecs::{prelude::Component, storage::DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender},
};

use crate::asset_loader::SpriteSheetHandle;

pub struct Dude {}

impl Dude {
    pub fn new() -> Dude {
        Dude {}
    }
}

impl Component for Dude {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_dude(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(32.0, 32.00, 0.0);

    let sprite_renderer = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Dude::new())
        .with(sprite_renderer)
        .with(transform)
        .build();
}