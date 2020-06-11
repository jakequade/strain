use amethyst::{
    assets::{Handle, Prefab},
    ecs::prelude::{Builder, World, WorldExt},
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    renderer::transparent::Transparent,
};

use crate::components::{
    animation::{Animation, AnimationId, AnimationPrefabData},
    Dude
};

pub fn load_dude(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>) {
    println!("loading dude");
    let mut transform = Transform::default();

    transform.set_translation_x(384.);
    transform.set_translation_y(176.);

    world.create_entity()
    .with(Dude::new())
    .named("Dude")
    .with(transform)
    .with(Animation::new(
        AnimationId::DudeIdle,
        vec![AnimationId::DudeIdle]
    ))
    .with(prefab)
    .with(Transparent)
    .build();
}