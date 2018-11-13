use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use Paddle;

/// This system is responsible for moving all the paddles according to the user provided input.
pub struct UserControlled;

impl<'s> System<'s> for UserControlled {
    type SystemData = (
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (paddles, mut transforms, time, input): Self::SystemData) {
        // Iterate over all planks and move them according to the input the user
        // provided.
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            if let Some(movement) = input.axis_value("vertical") {
                transform.translate_y(paddle.velocity * time.delta_seconds() * movement as f32);
            }
            if let Some(movement) = input.axis_value("horizontal") {
                transform.translate_x(paddle.velocity * time.delta_seconds() * movement as f32);
            }
        }
    }
}
