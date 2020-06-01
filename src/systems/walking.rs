use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::strain::{Dude, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(SystemDesc)]
pub struct WalkingSystem;

impl<'s> System<'s> for WalkingSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Dude>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (input, dudes, mut transforms): Self::SystemData) {
        for (_dude, transform) in (&dudes, &mut transforms).join() {
            if let Some(movement) = input.axis_value("dude_horizontal") {
                if movement != 0.0 {
                    let scaled_amount = 4.0 * movement as f32;

                    let clamped_x = transform.translation().x;

                    transform.set_translation_x(
                        (clamped_x + scaled_amount)
                            .min(WINDOW_WIDTH - 16.0)
                            .max(16.0),
                    );
                }
            }
        }
    }
}
