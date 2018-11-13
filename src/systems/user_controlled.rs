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
//            let opt_movement = match paddle.side {
//                Side::Left => input.axis_value("left_paddle"),
//                Side::Right => input.axis_value("right_paddle"),
//            };
            let opt_movement = input.axis_value("player_control");

            if let Some(movement) = opt_movement {
                use ARENA_HEIGHT;
                transform.translate_y(paddle.velocity * time.delta_seconds() * movement as f32);

                // We make sure the paddle remains in the arena.
                let paddle_y = transform.translation().y;
                transform.set_y(
                    paddle_y
                        .max(paddle.height * 0.5)
                        .min(ARENA_HEIGHT - paddle.height * 0.5),
                );
            }
        }
    }
}
