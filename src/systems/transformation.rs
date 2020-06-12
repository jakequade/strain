use amethyst::{
  core::Transform,
  ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
  components::{Dude, Subject},
  resources::Context
};

#[derive(Default)]
pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
  type SystemData = WriteStorage<'s, Transform>;

  fn run(&mut self, mut transforms: Self::SystemData) {}
}

//#region CameraTransformationSystem

pub struct CameraTransformationSystem;

impl<'s> System<'s> for CameraTransformationSystem {
  type SystemData = (
    ReadStorage<'s, Dude>,
    ReadStorage<'s, Subject>,
    WriteStorage<'s, Transform>,
    ReadExpect<'s, Context>,
  );

  fn run (&mut self, (dudes, subjects, mut transforms, contexts): Self::SystemData) {
    let mut dude_x = 0.;

    let map_width = 700.;
    let background_width = 700.;

    for (_dude, transform) in (&dudes, &mut transforms).join() {
      dude_x = transform.translation().x;

      // for (_subject, transform) in (&subjects, &mut transforms).join() {
      //   if (dude_x >= background_width && dude_x <= map_width - background_width) {
      //     transform.set_translation_x(dude_x);
      //   }
      // }
    }
  }
}

//#endregion