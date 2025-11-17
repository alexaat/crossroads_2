use super::*;
use crate::model::Car;
use crate::model::Destination;
use crate::model::Line;
use crate::model::VehicleType;
use crate::preferences::*;
use crate::ui::Text;
use crate::Model;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use std::collections::HashMap;

pub struct View<'a> {
    canvas: Canvas<Window>,
    bg_color: (u8, u8, u8),
    texture_manager: &'a TextureManager<'a>,
    font_manager: &'a FontManager<'a>,
}

impl<'a> View<'a> {
    pub fn new(
        canvas: Canvas<Window>,
        bg_color: (u8, u8, u8),
        texture_manager: &'a TextureManager,
        font_manager: &'a FontManager,
    ) -> View<'a> {
        View {
            canvas,
            bg_color,
            texture_manager,
            font_manager,
        }
    }

    pub fn draw_model(&mut self, model: &mut Model) {
        let (r, g, b) = self.bg_color;
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.clear();

        let carrigeway_width = (CAR_WIDTH * 3 + MARGIN * 6) * 2;
        let field_width = (SCREEN_WIDTH - carrigeway_width) / 2;
        let field_heigth = (SCREEN_HEIGHT - carrigeway_width) / 2;
        let center = Point::new((field_width / 2) as i32, (field_heigth / 2) as i32);
        let src = Rect::new(0, 0, field_width, field_heigth);

        //draw background top-left
        let dst = Rect::new(0, 0, field_width, field_heigth);
        match self.texture_manager.textures.get("top_left") {
            Some(texture) => {
                if let Err(_) = self
                    .canvas
                    .copy_ex(texture, src, dst, 0.0, center, false, false)
                {
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    self.canvas.fill_rect(dst).unwrap();
                }
            }
            None => {
                self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                self.canvas.fill_rect(dst).unwrap();
            }
        }
        //draw background bottom-left
        let dst = Rect::new(
            0,
            (field_heigth + carrigeway_width) as i32,
            field_width,
            field_heigth,
        );

        match self.texture_manager.textures.get("top_left") {
            Some(texture) => {
                if let Err(_) = self
                    .canvas
                    .copy_ex(texture, src, dst, 0.0, center, false, true)
                {
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    self.canvas.fill_rect(dst).unwrap();
                }
            }
            None => {
                self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                self.canvas.fill_rect(dst).unwrap();
            }
        }
        //draw background top-right
        let dst = Rect::new(
            (field_width + carrigeway_width) as i32,
            0,
            field_width,
            field_heigth,
        );

        match self.texture_manager.textures.get("top_left") {
            Some(texture) => {
                if let Err(_) = self
                    .canvas
                    .copy_ex(texture, src, dst, 0.0, center, true, false)
                {
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    self.canvas.fill_rect(dst).unwrap();
                }
            }
            None => {
                self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                self.canvas.fill_rect(dst).unwrap();
            }
        }
        //draw background bottom-right
        let dst = Rect::new(
            (field_width + carrigeway_width) as i32,
            (field_heigth + carrigeway_width) as i32,
            field_width,
            field_heigth,
        );

        match self.texture_manager.textures.get("top_left") {
            Some(texture) => {
                if let Err(_) = self
                    .canvas
                    .copy_ex(texture, src, dst, 0.0, center, true, true)
                {
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    self.canvas.fill_rect(dst).unwrap();
                }
            }
            None => {
                self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                self.canvas.fill_rect(dst).unwrap();
            }
        }
        //Draw road markings
        for line in &model.lines {
            line.draw(&mut self.canvas, None);
        }

        //Draw cars
        for car in &model.cars {
            car.draw(&mut self.canvas, Some(&self.texture_manager));
        }

        self.canvas.present();
    }

    pub fn draw_statistics(&mut self, model: &Model) {
        self.canvas.set_draw_color(Color::RGB(10, 10, 10));
        let _ = self.canvas.fill_rect(Rect::new(145, 195, 400, 300));

        let texture_creator = self.canvas.texture_creator();
        //title
        if let Some(font) = self.font_manager.fonts.get("title") {
            let _ = Text::render_text(
                &mut self.canvas,
                &texture_creator,
                font,
                "Statistics",
                Color::WHITE,
                (290, 220),
            );
        }

        if let Some(font) = self.font_manager.fonts.get("body") {
            let left = 165;
            //max num of vehicles
            let message = format!(
                "Max number of vehicles: {}",
                model.statistics.number_of_vehicles
            );
            let _ = Text::render_text(
                &mut self.canvas,
                &texture_creator,
                &font,
                &message,
                Color::WHITE,
                (left, 280),
            );
            //max velocity
            let mut max: f32 = 0.0;
            if let Some(m) = model.statistics.max_velocity {
                max = m;
            }
            let message = format!("Max velocity: {}", max);
            let _ = Text::render_text(
                &mut self.canvas,
                &texture_creator,
                &font,
                &message,
                Color::WHITE,
                (left, 310),
            );
            //min velocity
            let mut min: f32 = 0.0;
            if let Some(m) = model.statistics.min_velocity {
                min = m;
            }
            let message = format!("Min velocity: {}", min);
            let _ = Text::render_text(
                &mut self.canvas,
                &texture_creator,
                &font,
                &message,
                Color::WHITE,
                (left, 340),
            );
            //max time
            let mut time: u128 = 0;
            if let Some(max) = model.statistics.max_time {
                time = max;
            }
            let message = format!("Max time: {} ms", time);
            let _ = Text::render_text(
                &mut self.canvas,
                &texture_creator,
                &font,
                &message,
                Color::WHITE,
                (left, 370),
            );
            //min time
            let mut time: u128 = 0;
            if let Some(min) = model.statistics.min_time {
                time = min;
            }
            let message = format!("Min time: {} ms", time);
            let _ = Text::render_text(
                &mut self.canvas,
                &texture_creator,
                &font,
                &message,
                Color::WHITE,
                (left, 400),
            );
        }

        self.canvas.present();
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &mut Canvas<Window>, texture_manager: Option<&TextureManager>) {
        let (r, g, b) = self.color;
        canvas.set_draw_color(Color::RGB(r, g, b));
        let start = Point::new(self.start.x, self.start.y);
        let end = Point::new(self.end.x, self.end.y);
        canvas.draw_line(start, end).unwrap();
    }
}

impl Drawable for Car {
    fn draw(&self, canvas: &mut Canvas<Window>, texture_manager: Option<&TextureManager>) {
        let (r, g, b) = (255, 0, 0);

        canvas.set_draw_color(Color::RGB(r, g, b));

        match texture_manager {
            Some(texture_manager) => {}
            None => {}
        }

        let texture_creator = canvas.texture_creator();
        let url = match self.vehicle_type {
            VehicleType::BlueCar => CAR_URLS[0],
            VehicleType::GreenCar => CAR_URLS[2],
            VehicleType::RedCar => CAR_URLS[1],
        };
        let texture = texture_creator.load_texture(url).unwrap();
        let src = Rect::new(0, 0, CAR_LENGTH_DEFAULT, CAR_WIDTH_DEFAULT);

        let x = self.center.x as i32 - CAR_LENGTH_I32 / 2;
        let y = self.center.y as i32 - CAR_WIDTH_I32 / 2;
        let dst = Rect::new(x, y, CAR_LENGTH, CAR_WIDTH);
        let center = Point::new(CAR_LENGTH_I32 / 2, CAR_WIDTH_I32 / 2);

        canvas
            .copy_ex(
                &texture,
                src,
                dst,
                self.rotation as f64,
                center,
                false,
                false,
            )
            .unwrap();

        // match self.destination {
        //     Destination::LEFT => canvas.set_draw_color(Color::RGB(0, 0, 255)),
        //     Destination::AHEAD => canvas.set_draw_color(Color::RGB(0, 255, 0)),
        //     Destination::RIGHT => canvas.set_draw_color(Color::RGB(255, 0, 0)),
        // };
    }
}

trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>, texture_manager: Option<&TextureManager>);
}

pub struct TextureManager<'a> {
    pub textures: HashMap<String, Texture<'a>>,
}
impl<'a> TextureManager<'a> {
    pub fn new() -> TextureManager<'a> {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        name: &str,
        path: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) {
        match texture_creator.load_texture(path) {
            Ok(texture) => {
                self.textures.insert(name.to_string(), texture);
            }
            Err(e) => println!("cannot load texture: {:?}", e),
        }
    }
}

pub struct FontManager<'ttf> {
    pub fonts: HashMap<String, Font<'ttf, 'ttf>>,
}

impl<'ttf> FontManager<'ttf> {
    pub fn new() -> FontManager<'ttf> {
        FontManager {
            fonts: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, path: &str, size: u16, ttf: &'ttf Sdl2TtfContext) {
        match ttf.load_font(path, size) {
            Ok(font) => {
                self.fonts.insert(name.to_string(), font);
            }
            Err(e) => println!("cannot load font: {:?}", e),
        }
    }
}
