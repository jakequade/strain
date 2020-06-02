use amethyst::{
    ecs::{prelude::Component, storage::DenseVecStorage},
    prelude::*
};

#[derive(Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    fn new(x: f32, y: f32) -> Velocity {
        Velocity { x, y }
    }
}

impl Component for Velocity {
    // TODO: this probably doesn't need to be dense
    type Storage = DenseVecStorage<Self>;
}
