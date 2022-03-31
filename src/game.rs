use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub struct Game<'canvas> {
    canvas: &'canvas mut WindowCanvas,
}

impl<'canvas> Game<'canvas> {
    pub fn new(canvas: &mut WindowCanvas) -> Game {
        Game { canvas }
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
