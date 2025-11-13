use crate::model::Line;
use crate::Model;
use crate::preferences::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;


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


        let carrigeway_width = (CAR_WIDTH * 3 + MARGIN * 6) * 2;
        let field_width = (SCREEN_WIDTH - carrigeway_width) / 2;
        let field_heigth = (SCREEN_HEIGHT - carrigeway_width) / 2;

        //draw background top-left
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(TOP_LEFT_URL).unwrap();
        let src = Rect::new(0, 0, field_width, field_heigth);

        let dst = Rect::new(0, 0, field_width, field_heigth);
        let center = Point::new((field_width / 2) as i32, (field_heigth / 2) as i32);

        self.canvas
            .copy_ex(&texture, src, dst, 0.0, center, false, false)
            .unwrap();




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
