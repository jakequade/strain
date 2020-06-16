use amethyst::ecs::{prelude::Component, storage::DenseVecStorage};

pub enum DudeState {
  Idle,
  Walking,
}

pub struct Dude {
  pub state: DudeState,
}

impl Dude {
  pub fn new() -> Dude {
    Dude {
      state: DudeState::Idle,
    }
  }
}

impl Component for Dude {
  type Storage = DenseVecStorage<Self>;
}
