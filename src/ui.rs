use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

pub struct Text {}
impl Text {
    pub fn render_text<'a>(
        canvas: &mut sdl2::render::Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
        font: &'a Font,
        text: &str,
        color: Color,
        (x, y): (i32, i32),
    ) -> Result<Texture<'a>, String> {
        let surface = font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(x, y, width, height);
        canvas.copy(&texture, None, Some(target))?;
        Ok(texture)
    }
}