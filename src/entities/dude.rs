use amethyst::{
    assets::{Handle, Prefab},
    core::{Transform, WithNamed},
    ecs::prelude::{Builder, World, WorldExt},
    renderer::transparent::Transparent,
};

use crate::components::{
    animation::{Animation, AnimationId, AnimationPrefabData},
    direction::Direction,
    dude::Dude,
    motion::Motion,
};

pub fn load_dude(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>) {
    let mut transform = Transform::default();
    let motion = Motion::new();

    transform.set_translation_x(384.);
    transform.set_translation_y(176.);

    world
        .create_entity()
        .with(Direction::default())
        .with(Dude::new())
        .named("Dude")
        .with(Animation::new(
            AnimationId::DudeIdle,
            vec![AnimationId::DudeIdle, AnimationId::DudeWalking],
        ))
        .with(motion)
        .with(prefab)
        .with(transform)
        .with(Transparent)
        .build();
}
