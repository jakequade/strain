use amethyst::ecs::{prelude::Component, storage::DenseVecStorage};

pub enum DudeState {
  Idle,
  Walking,
}

pub struct Dude {
  pub max_ground_speed: f32,
  pub state: DudeState,
}

impl Dude {
  pub fn new() -> Dude {
    Dude {
      max_ground_speed: 6.,
      state: DudeState::Idle,
    }
  }
}

impl Component for Dude {
  type Storage = DenseVecStorage<Self>;
}
