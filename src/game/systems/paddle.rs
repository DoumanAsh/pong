use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::core::timing::Time;

use game::components::{Side, Paddle, PADDLE_HEIGHT};
use game::ARENA_HEIGHT;

pub struct PaddleSystem;

pub const NAME: &'static str = "paddle_system";

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Paddle>, Read<'s, InputHandler<String, String>>, Read<'s, Time>);

    fn run(&mut self, (mut transforms, paddles, input, time): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            match movement {
                Some(mv_amount) if mv_amount != 0.0 => {
                    //Simply scale a bit movement
                    let mv_amount = (1.0 + time.delta_seconds()) * mv_amount as f32;
                    //Movement should be limited within our screen
                    //min should just a bit lower top of the screen
                    //max should be just a bit above of the top
                    let mv_amount = (transform.translation[1] + mv_amount).min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5).max(PADDLE_HEIGHT * 0.5);
                    //1 is position on Y
                    transform.translation[1] = mv_amount;
                },
                _ => (),
            }
        }
    }
}
