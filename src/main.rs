// https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html
mod snake;
mod direction;
mod game;
mod point;
mod command;

use crate::game::Game;
use std::io::stdout;

fn main() {
    Game::new(stdout(), 10, 10).run();
}
