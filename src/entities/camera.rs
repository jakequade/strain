use amethyst::{
    core::{Parent, Transform},
    ecs::{prelude::{Builder, World, WorldExt}, Entity},
    renderer::Camera,
    window::ScreenDimensions
};

pub fn load_camera(world: &mut World, subject: Entity) {
    let (height, width) = {
        let dimensions = world.fetch::<ScreenDimensions>();
        (dimensions.height(), dimensions.width())
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
      .create_entity()
      .with(Camera::standard_2d(width, height))
      .with(Parent {
          entity: subject
      })
      .with(transform)
      .build();
}