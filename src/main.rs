extern crate amethyst;
extern crate lazy_panic;
extern crate log;
extern crate rand;

mod rt;
mod utils;
mod game;

fn main() {
    rt::init();

    game::run().expect("Run game successfully");
}
