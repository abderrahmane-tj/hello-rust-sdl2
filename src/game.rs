use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::time::Duration;

pub struct Game<'sdl> {
    sdl: &'sdl mut Sdl,
    canvas: WindowCanvas,
}

impl<'sdl> Game<'sdl> {
    pub fn new(sdl: &mut Sdl) -> Game {
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Game { sdl, canvas }
    }

    pub fn start(self: &mut Self) {
        let mut event_pump = self.sdl.event_pump().unwrap();

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
            self.draw();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn draw_background(self: &mut Self) {
        self.canvas.set_draw_color(Color::RGB(67, 139, 220));
        self.canvas.clear();
    }

    pub fn draw_player(self: &mut Self) {
        self.canvas.set_draw_color(Color::RGB(67, 220, 139));
        self.canvas.fill_rect(Rect::new(10, 10, 140, 140)).unwrap();
    }

    pub fn draw(self: &mut Self) {
        self.draw_background();
        self.draw_player();
        self.canvas.present();
    }
}
