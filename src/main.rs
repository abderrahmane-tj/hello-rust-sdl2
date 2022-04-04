extern crate sdl2;

pub mod game;

use crate::game::Game;

pub fn main() {
    let mut sdl = sdl2::init().unwrap();

    let mut game = Game::new(&mut sdl);
    game.start();
}
