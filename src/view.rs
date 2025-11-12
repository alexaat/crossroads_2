use crate::model::Line;
use crate::Model;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct View {
    canvas: Canvas<Window>,
    bg_color: (u8, u8, u8),
}

impl View {
    pub fn new(canvas: Canvas<Window>, bg_color: (u8, u8, u8)) -> Self {
        Self { canvas, bg_color }
    }

    pub fn draw_model(&mut self, model: &mut Model) {
        let (r, g, b) = self.bg_color;
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.clear();

        //Draw road markings
        for line in &model.lines {
            line.draw(&mut self.canvas);
        }

        self.canvas.present();
    }

    pub fn draw_statistics(&mut self, model: &Model) {}
}

impl Drawable for Line {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        let (r, g, b) = self.color;
        canvas.set_draw_color(Color::RGB(r, g, b));
        let start = Point::new(self.start.x, self.start.y);
        let end = Point::new(self.end.x, self.end.y);
        canvas.draw_line(start, end).unwrap();
    }
}

trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
