use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Dude, velocity::Velocity};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (WriteStorage<'s, Dude>, ReadStorage<'s, Velocity>);

    fn run(&mut self, (mut dudes, velocities): Self::SystemData) {
        for (mut dude, velocity) in (&mut dudes, &velocities).join() {
            println!("{:?}", velocity);
        }
    }
}
