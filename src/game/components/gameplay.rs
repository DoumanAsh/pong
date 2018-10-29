use amethyst::prelude::{Builder, World};
use amethyst::ecs::prelude::{DenseVecStorage, Component};
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;

pub const BALL_NUM: usize = 1;
const BALL_VELOCITY_X: f32 = 50.0;
const BALL_VELOCITY_Y: f32 = 25.0;
const BALL_RADIUS: f32 = 2.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;

use game::{ARENA_HEIGHT_MIDDLE, ARENA_WIDTH, ARENA_WIDTH_MIDDLE};

pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn is_left(&self) -> bool {
        match self {
            Side::Left => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Side::Right => true,
            _ => false,
        }
    }
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Self {
        Paddle {
            side: side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }

    fn left() -> Self {
        Self::new(Side::Left)
    }

    fn right() -> Self {
        Self::new(Side::Right)
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
            radius: BALL_RADIUS,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_paddles(world: &mut World, sprite: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    //Prepare transform of position.
    //Middle of the screen is our anchor
    //therefore we position paddles relative to it
    left_transform.translation = Vector3::new(PADDLE_WIDTH * 0.5, ARENA_HEIGHT_MIDDLE, 0.0);
    right_transform.translation = Vector3::new(ARENA_WIDTH - PADDLE_WIDTH * 0.5, ARENA_HEIGHT_MIDDLE, 0.0);

    let sprite_left = SpriteRender {
        sprite_sheet: sprite.clone(),
        sprite_number: 0, // Refer to paddle texture ron, it is first.
        flip_horizontal: false,
        flip_vertical: false,
    };

    let sprite_right = SpriteRender {
        sprite_sheet: sprite,
        sprite_number: 0,
        flip_horizontal: true,
        flip_vertical: false,
    };

    //We create only entities with our paddles
    //and attach sprites to them
    world.create_entity_unchecked()
         .with(sprite_left)
         .with(Paddle::left())
         .with(left_transform)
         .build();

    world.create_entity_unchecked()
         .with(sprite_right)
         .with(Paddle::right())
         .with(right_transform)
         .build();
}

pub fn init_ball(world: &mut World, sprite: SpriteSheetHandle) {
    //Place ball at exact center
    let mut transform = Transform::default();

    transform.translation = Vector3::new(ARENA_WIDTH_MIDDLE, ARENA_HEIGHT_MIDDLE, 0.0);

    let sprite = SpriteRender {
        sprite_sheet: sprite,
        sprite_number: 1, //Ball is the second sprite in the ron config.
        flip_horizontal: true,
        flip_vertical: false,
    };

    world.create_entity_unchecked()
         .with(sprite)
         .with(Ball::default())
         .with(transform)
         .build();
}