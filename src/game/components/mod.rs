use amethyst::prelude::{World};

pub mod gameplay;
pub mod utils;
pub mod ui;

pub fn initialize(world: &mut World) {
    gameplay::init_paddles(world);
    gameplay::init_ball(world);
    utils::init_pop_up(world);

    ui::create_images(world);
    let font = ui::get_default_font(world);
    ui::init_score_board(world, font.clone());
}
