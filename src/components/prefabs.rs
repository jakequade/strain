use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, AnimationSetPrefab, EndControl,
    },
    assets::{PrefabData, PrefabLoader, PrefabLoaderSystemDesc, ProgressCounter, RonFormat},
    core::transform::Transform,
    derive::PrefabData,
    ecs::{prelude::Entity, Entities, Join, ReadStorage, WriteStorage},
    error::Error,
    prelude::*,
    renderer::sprite::{prefab::SpriteScenePrefab, SpriteRender},
    GameData, SimpleTrans, StateData,
};
use serde::{Deserialize, Serialize};

use crate::strain::Strain;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
enum AnimationId {
    Idle,
}

#[derive(Clone, Debug, Deserialize, PrefabData)]
struct AnimationPrefabData {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

pub fn init_idle_animation(world: &mut World, data: &mut Strain) {
    let dude_idle_prefab = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load(
            "prefabs/dude_animation.ron",
            RonFormat,
            data.progress_counter.as_mut().unwrap()
        )
    });
    world.create_entity().with(dude_idle_prefab).build();
}

pub fn update_idle_animation(world: &mut World, data: &mut StateData<'_, GameData<'_, '_>>, strain_instance: &mut Strain) -> SimpleTrans {
    // Checks that loading is still happening
    if let Some(ref progress_counter) = strain_instance.progress_counter {
        if progress_counter.is_complete() {
            let StateData { world, .. } = data;

            world.exec(
                |(entities, animation_sets, mut control_sets): (
                    Entities,
                    ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                    WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
                )| {
                    // For each entity with an AnimationSet
                    for (entity, animation_set) in (&entities, &animation_sets).join() {
                        // Create the AnimationControlSet
                        let control_set = get_animation_set(&mut control_sets, entity).unwrap();

                        control_set.add_animation(
                            AnimationId::Idle,
                            &animation_set.get(&AnimationId::Idle).unwrap(),
                            EndControl::Loop(None),
                            1.0,
                            AnimationCommand::Start
                        );
                    }
                },
            );
            // All data has loaded
            strain_instance.progress_counter = None;
        }
    }
    Trans::None
}
