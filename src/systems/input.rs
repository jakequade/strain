use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{
    direction::{Direction, Directions},
    dude::{Dude, DudeState},
};

pub struct DudeInputSystem;

impl<'s> System<'s> for DudeInputSystem {
    type SystemData = (
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Dude>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut directions, mut dudes, input): Self::SystemData) {
        let run_input = input
            .axis_value("dude_horizontal")
            .expect("Could not find dude_horizontal axis value");

        

        for (direction, dude) in (&mut directions, &mut dudes).join() {
            if run_input == 0.0 {
                dude.state = DudeState::Idle;
                return;
            }

            if run_input < 0.0 {
                direction.x = Some(Directions::Left);
                dude.state = DudeState::Walking;
            }
            if run_input > 0.0 {
                direction.x = Some(Directions::Right);
                dude.state = DudeState::Walking;
            }
        }
    }
}
