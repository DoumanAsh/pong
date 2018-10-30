extern crate amethyst;
extern crate lazy_panic;
extern crate log;
extern crate rand;
extern crate cute_log;

mod rt;
mod utils;
mod game;

fn main() {
    rt::init();

    game::run().expect("Run game successfully");
}
