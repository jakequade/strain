use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    components::dude::Dude,
    strain::{DUDE_HEIGHT_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
};

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
            // Horizontal movement
            if let Some(x_movement) = input.axis_value("dude_horizontal") {
                if x_movement != 0.0 {
                    let scaled_amount = 4.0 * x_movement as f32;

                    let clamped_x = transform.translation().x;

                    transform.set_translation_x(
                        (clamped_x + scaled_amount)
                            .min(WINDOW_WIDTH - DUDE_HEIGHT_WIDTH)
                            .max(DUDE_HEIGHT_WIDTH),
                    );
                }
            }

            // Vertical movement
            if let Some(y_movement) = input.axis_value("dude_vertical") {
                if y_movement != 0.0 {
                    let scaled_amount = 4.0 * y_movement as f32;

                    let clamped_y = transform.translation().y;

                    transform.set_translation_y(
                        (clamped_y + scaled_amount)
                            .min(WINDOW_HEIGHT - DUDE_HEIGHT_WIDTH)
                            .max(DUDE_HEIGHT_WIDTH),
                    );
                }
            }
        }
    }
}
