use amethyst::{
    assets::{Handle, Prefab},
    core::{math::Vector3, Transform, WithNamed},
    ecs::prelude::{Builder, World, WorldExt},
    renderer::transparent::Transparent,
};

use crate::components::{
    animation::{Animation, AnimationId, AnimationPrefabData},
    direction::Direction,
    dude::Dude,
    motion::Motion,
};

const SCALE_MAGNITUDE: f32 = 2.;

pub fn load_dude(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>) {
    let mut transform = Transform::default();
    let motion = Motion::new();

    transform.set_scale(Vector3::new(
        SCALE_MAGNITUDE,
        SCALE_MAGNITUDE,
        SCALE_MAGNITUDE,
    ));
    transform.set_translation_x(384.);
    transform.set_translation_y(176.);

    world
        .create_entity()
        .with(Direction::default())
        .with(Dude::new())
        .named("Dude")
        .with(Animation::new(
            AnimationId::DudeIdle,
            vec![
                AnimationId::DudeIdle,
                AnimationId::DudeJumping,
                AnimationId::DudeWalking,
            ],
        ))
        .with(motion)
        .with(prefab)
        .with(transform)
        .with(Transparent)
        .build();
}
