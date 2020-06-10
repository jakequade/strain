use amethyst::{
  core::Transform,
  ecs::{System, WriteStorage},
};

#[derive(Default)]
pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
  type SystemData = WriteStorage<'s, Transform>;

  fn run(&mut self, mut transforms: Self::SystemData) {}
}
