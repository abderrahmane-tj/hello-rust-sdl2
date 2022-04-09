use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::RootContext;

pub struct Game<'t> {
    texture: Texture<'t>,
    canvas: &'t mut WindowCanvas,
}

impl<'t> Game<'t> {
    pub fn new(root_context: &'t mut RootContext) -> Game<'t> {
        let texture = root_context
            .texture_creator
            .create_texture_target(None, 600, 10)
            .unwrap();
        Game {
            texture,
            canvas: root_context.canvas,
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
