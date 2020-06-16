use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::dude::{Dude, DudeState};

pub enum Direction {
    Left,
    Right,
}

pub struct DudeInputSystem;

impl<'s> System<'s> for DudeInputSystem {
    type SystemData = (
        // WriteStorage<'s, Direction>,
        WriteStorage<'s, Dude>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut dudes, input): Self::SystemData) {
        // let run_input = input.axis_value("dude_x").expect("Could not find dude_x value");

        // for dude in (&mut dudes).join() {
        //     if run_input != 0.0 {
        //         dude.state = DudeState::Walking;
        //     }
        // }
    }
}
