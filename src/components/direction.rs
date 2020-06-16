use amethyst::ecs::{Component, DenseVecStorage};

pub enum Directions {
  Left,
  Right,
}

#[derive(Component)]
pub struct Direction {
  pub x: Option<Directions>,
  pub y: Option<Directions>,
}

impl Default for Direction {
  fn default() -> Self {
    Direction { x: None, y: None }
  }
}

impl Direction {
  pub fn new(x: Directions, y: Directions) -> Direction {
    Direction {
      x: Some(x),
      y: Some(y),
    }
  }

  pub fn set_x_velocity(&mut self, x_velocity: f32) {
    self.x = if x_velocity.abs() < std::f32::EPSILON {
      None
    } else if x_velocity > 0. {
      Some(Directions::Right)
    } else {
      Some(Directions::Left)
    }
  }
}
