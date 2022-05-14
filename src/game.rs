use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use crate::RootContext;

pub struct Game<'t> {
    texture: Texture<'t>,
    canvas: &'t mut WindowCanvas,
}

impl<'t> Game<'t> {
    pub fn new(root_context: &'t mut RootContext) -> Game<'t> {
        let mut texture = root_context
            .texture_creator
            .create_texture_target(None, 100, 100)
            .unwrap();

        root_context
            .canvas
            .with_texture_canvas(&mut texture, |canvas| {
                canvas.set_draw_color(Color::RGB(50, 150, 250));
                canvas
                    .draw_line(Point::new(5, 5), Point::new(50, 50))
                    .unwrap();
            })
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
        let playerSrc = Rect::new(10, 10, 140, 140);
        self.canvas.set_draw_color(Color::RGB(67, 220, 139));
        self.canvas.fill_rect(playerSrc).unwrap();

        self.canvas
            .copy(
                &self.texture,
                None,
                Rect::new(playerSrc.x + 10, playerSrc.y + 10, 100, 100),
            )
            .unwrap();
    }

    pub fn draw(self: &mut Self) {
        self.draw_background();
        self.draw_player();
        self.canvas.present();
    }
}
