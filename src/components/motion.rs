use amethyst::{
  core::math::Vector2,
  ecs::{Component, DenseVecStorage},
};

use crate::components::direction::{Direction, Directions};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Motion {
  pub is_midair: bool,
  pub velocity: Vector2<f32>,
}

impl Motion {
  pub fn new() -> Motion {
    Motion {
      is_midair: false,
      velocity: Vector2::new(0., 0.),
    }
  }

  pub fn update_velocity(
    &mut self,
    acceleration: Vector2<f32>,
    direction: &Direction,
    min_limit: f32,
    max_limit: f32,
  ) {
    match direction.x {
      None => {}
      Some(Directions::Left) => {
        self.velocity.x -= acceleration.x;

        if acceleration.x <= 0. {
          self.velocity.x = self.velocity.x.min(-min_limit);
        } else {
          self.velocity.x = self.velocity.x.max(-max_limit);
        }
      }
      Some(Directions::Right) => {
        if acceleration.x >= 0. {
          self.velocity.x = self.velocity.x.max(min_limit);
        } else {
          self.velocity.x = self.velocity.x.max(-max_limit);
        }
      }
    }
  }
}
