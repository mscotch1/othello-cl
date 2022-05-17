use std::env;

mod game;
mod cli;
mod display;
mod config;

fn main() {
    game::Game::new(&env::args()).run();
}
