use amethyst::prelude::{World};

pub mod gameplay;
pub mod ui;

pub fn initialize(world: &mut World) {
    //Now we load sprites for them and render them on screen
    let sprite = super::sprites::load_paddle(world);

    gameplay::init_paddles(world, sprite.clone());
    gameplay::init_ball(world, sprite);
    ui::init_score_board(world);
}
