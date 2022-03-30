use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Game {}

impl Game {
    pub fn new(canvas: &mut WindowCanvas) -> Game {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        Game {}
    }
    pub fn draw(self: &Self, canvas: &mut WindowCanvas, i: &mut u8) {
        *i = (*i + 1) % 255;
        canvas.set_draw_color(Color::RGB(*i, 64, 255 - *i));
        canvas.clear();
        canvas.present();
    }
}
