use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use std::f32::consts::PI;

use crate::components::direction::{Direction, Directions};

// Responsible for flipping components to face the direction they are moving in.
pub struct DirectionSystem;

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Direction>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, directions, mut transforms): Self::SystemData) {
        for (_, direction, transform) in (&entities, &directions, &mut transforms).join() {
            match direction.x {
                None => {
                    transform.set_rotation_y_axis(0.);
                },
                Some(Directions::Right) => {
                    transform.set_rotation_y_axis(0.);
                },
                Some(Directions::Left) => {
                    transform.set_rotation_y_axis(PI);
                }
            }
        }
    }
}