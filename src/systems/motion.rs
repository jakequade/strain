use amethyst::{
    core::math::Vector2,
    ecs::{Join, ReadStorage, System, WriteStorage}
};

use crate::components::{direction::Direction, dude::{Dude, DudeState}, motion::Motion};

/// Actions the changes in input and state that relate to movement.
pub struct MotionSystem;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Dude>,
        WriteStorage<'s, Motion>,
    );

    fn run(&mut self, (directions, dudes, mut motions): Self::SystemData) {
        for (direction, dude, motion) in (&directions, &dudes, &mut motions).join() {
            let acceleration = match dude.state {
                DudeState::Idle => {
                    let acceleration_x = if motion.velocity.x != 0. { -0.6 } else { 0. };
                    Vector2::new(acceleration_x, -0.6)
                },
                DudeState::Walking => {
                    Vector2::new(0.6, -0.6)
                }
            };

            motion.update_velocity(acceleration, direction, 0., dude.max_ground_speed);
        }
    }
}
