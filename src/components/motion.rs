use amethyst::{
  core::math::Vector2,
  ecs::{Component, DenseVecStorage},
};

use crate::components::direction::{Direction, Directions};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Motion {
  pub is_mid_air: bool,
  pub velocity: Vector2<f32>,
}

impl Motion {
  pub fn new() -> Self {
    Motion {
      is_mid_air: false,
      velocity: Vector2::new(0., 0.),
    }
  }

  /// Updates the velocity of a Motion.
  /// When moving right:
  /// - if the acceleration is 0. or less, we cap velocity max to the mininum.
  /// - if it's greater than 0., the velocity min is floored at the max limit.
  /// When moving left:
  /// - if the acceleration is 0. or less, velocity min is floored at the min_limit, negated.
  /// - if it's greater than 0., velocity max is is capped at the max limit, negated.
  pub fn update_velocity(
    &mut self,
    acceleration: Vector2<f32>,
    direction: &Direction,
    min_limit: f32,
    max_limit: f32,
  ) {
    match direction.x {
      Some(Directions::Right) => {
        self.velocity.x += acceleration.x;

        if acceleration.x <= 0. {
          self.velocity.x = self.velocity.x.max(min_limit);
        } else {
          self.velocity.x = self.velocity.x.min(max_limit);
        }
      },
      Some(Directions::Left) => {
        self.velocity.x -= acceleration.x;

        if acceleration.x <= 0. {
          self.velocity.x = self.velocity.x.min(-min_limit);
        } else {
          self.velocity.x = self.velocity.x.max(-max_limit);
        }
      }
      None => {}
    }
  }
}
