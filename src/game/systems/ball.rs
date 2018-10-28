use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use utils;
use game::components::{BALL_NUM, Paddle, Ball};
use game::{ARENA_WIDTH_MIDDLE, ARENA_HEIGHT_MIDDLE, ARENA_WIDTH, ARENA_HEIGHT};

pub const MOVE: &'static str = "ball_move_system";
pub const COLLISION: &'static str = "ball_collision_system";

pub struct BallMove;

impl<'s> System<'s> for BallMove {
    type SystemData = (ReadStorage<'s, Ball>, WriteStorage<'s, Transform>, Read<'s, Time>);

    fn run(&mut self, (balls, mut transforms, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (ball, transforms) in (&balls, &mut transforms).join() {
            transforms.translation[0] += ball.velocity[0] * time.delta_seconds();
            transforms.translation[1] += ball.velocity[1] * time.delta_seconds();
        }
    }
}

pub struct BallCollision;

impl BallCollision {
    fn reset_ball(ball: &mut Ball, transform: &mut Transform) {
        let rnd_idx = utils::random_rng(0, 2);
        ball.velocity[rnd_idx] = -ball.velocity[rnd_idx];

        transform.translation[0] = ARENA_WIDTH_MIDDLE;
        transform.translation[1] = ARENA_HEIGHT_MIDDLE;
    }

    #[inline(always)]
    fn is_ball_collide_with_paddle(ball: &Ball, pos: &(f32, f32), paddle: &Paddle, paddle_transform: &Transform) -> bool {
        //Center position of paddle in the middle of it.
        let paddle_pos = (paddle_transform.translation[0] - paddle.width * 0.5, paddle_transform.translation[1] - paddle.height * 0.5);

        //Prepare collision area rectangle coords
        let left = paddle_pos.0 - ball.radius;
        let bottom = paddle_pos.1 - ball.radius;
        let right = paddle_pos.0 + paddle.width + ball.radius;
        let top = paddle_pos.1 + paddle.height + ball.radius;

        //Are we within collision rectangle?
        if pos.0 >= left && pos.0 <= right && pos.1 >= bottom && pos.1 <= top {
            if paddle.side.is_left() && ball.velocity[0] < 0.0 {
                return true;
            } else if paddle.side.is_right() && ball.velocity[0] > 0.0 {
                return true;
            }
        }

        false
    }
}

impl<'s> System<'s> for BallCollision {
    type SystemData = (WriteStorage<'s, Ball>, ReadStorage<'s, Paddle>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut balls, paddles, mut transforms): Self::SystemData) {
        let mut balls_passed = [true; BALL_NUM];
        let mut ball_idx = 0;

        //We need to through to check whether ball collides with anything
        'ball: for (ball, transform) in (&mut balls, &transforms).join() {
            ball_idx += 1;
            let pos = (transform.translation[0], transform.translation[1]);

            //First check if we reached the top or bottom edges of arena
            //When we reach left or right edges, we negate current velocity
            //to switch direction
            if pos.1 <= ball.radius && ball.velocity[1] < 0.0 {
                ball.velocity[1] = -ball.velocity[1];
                continue;
            } else if pos.1 >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1];
                continue;
            } else {
                for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                    if Self::is_ball_collide_with_paddle(ball, &pos, paddle, paddle_transform) {
                        ball.velocity[0] = -ball.velocity[0];
                        continue 'ball;
                    }
                }
            }

            balls_passed[ball_idx-1] = false;
        }

        ball_idx = 0;

        for (ball, transform) in (&mut balls, &mut transforms).join() {
            if balls_passed[ball_idx] == false {
                let pos = (transform.translation[0], transform.translation[1]);

                //TODO: Implement game over? Or restart?
                //left edge 0, right 100
                //Stop ball if we reach left or right edges
                if pos.0 <= ball.radius && ball.velocity[0] < 0.0 {
                    Self::reset_ball(ball, transform);
                } else if pos.0 >= ARENA_WIDTH - ball.radius && ball.velocity[0] > 0.0 {
                    Self::reset_ball(ball, transform);
                }
            }

            ball_idx += 1;
        }
    }
}
