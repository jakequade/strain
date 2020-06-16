use amethyst::{
  core::math::Vector2,
  ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{
  direction::Direction,
  dude::{Dude, DudeState},
  motion::Motion,
};

pub struct DudeMotionSystem;

impl<'s> System<'s> for DudeMotionSystem {
  type SystemData = (
    ReadStorage<'s, Direction>,
    ReadStorage<'s, Dude>,
    WriteStorage<'s, Motion>,
  );

  fn run(&mut self, (directions, dudes, mut motions): Self::SystemData) {
    for (direction, dude, motion) in (&directions, &dudes, &mut motions).join() {
      let mut acceleration = Vector2::new(0., 0.);

      match dude.state {
        DudeState::Idle => {
          let acceleration_x = if motion.velocity.x != 0.0 { -0.6 } else { 0.0 };
          acceleration = Vector2::new(acceleration_x, -0.6)
        }
        DudeState::Walking => {
          acceleration = Vector2::new(0.6, -0.6);
        } // TODO: Add jumping and dead states.
      }

      // motion.update_velocity(acceleration, 0., dude.max_ground_speed);
    }
  }
}
