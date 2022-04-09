extern crate sdl2;

pub mod game;
pub mod screen;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::game::Game;

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let tc = canvas.texture_creator();

    let mut game = Game::new(&tc, canvas);

    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        game.draw();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
