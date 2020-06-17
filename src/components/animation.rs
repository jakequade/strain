use amethyst::{
  assets::{PrefabData, ProgressCounter},
  animation::AnimationSetPrefab,
  derive::PrefabData,
  ecs::{Component, DenseVecStorage, Entity},
  error::Error,
  renderer::sprite::{prefab::SpriteScenePrefab, SpriteRender}
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum AnimationId {
  DudeIdle,
  DudeJumping,
  DudeWalking,
}

//#region Animation

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Animation {
  pub current: AnimationId,
  pub show: bool,
  pub types: Vec<AnimationId>,
}

impl Animation {
  pub fn new(current: AnimationId, types: Vec<AnimationId>) -> Self {
    Self {
      current,
      show: true,
      types,
    }
  }
}

//#endregion

/// Used to load `SpriteScene`s and their `AnimationSet`s
#[derive(Clone, Debug, Deserialize, PrefabData)]
pub struct AnimationPrefabData {
  /// Information for rendering a scene with sprites
  sprite_scene: SpriteScenePrefab,
  /// All animations that can be run on the `Entity`
  animation_set: AnimationSetPrefab<AnimationId, SpriteRender>
}