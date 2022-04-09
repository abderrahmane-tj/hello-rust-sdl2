extern crate sdl2;

pub mod game;
pub mod screen;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::Sdl;
use sdl2::video::WindowContext;
use crate::game::Game;

pub struct RootContext<'r> {
    canvas: &'r mut WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    sdl: &'r Sdl,
}

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let root_context = &mut RootContext {
        canvas: &mut canvas,
        texture_creator,
        sdl: &sdl,
    };
    let mut game = Game::new(root_context);

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
