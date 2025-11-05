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
}
