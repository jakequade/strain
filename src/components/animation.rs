use amethyst::{
  derive::PrefabData,
  ecs::{Component, DenseVecStorage},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum AnimationId {
  DudeIdle,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Animation {
  pub current: AnimationId,
  pub show: bool,
  pub types: Vec<AnimationId>,
}
