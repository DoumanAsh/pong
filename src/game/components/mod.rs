use amethyst::prelude::{World};

pub mod gameplay;
pub mod ui;

pub fn initialize(world: &mut World) {
    gameplay::init_paddles(world);
    gameplay::init_ball(world);
    ui::init_score_board(world);
}
