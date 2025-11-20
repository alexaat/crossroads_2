use super::*;
use crate::model::*;
use crate::preferences::*;
use crate::ui::Text;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
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

        self.draw_background(&model.lines);

        //Draw cars
        for car in &model.cars {
            car.draw(&mut self.canvas, Some(&self.texture_manager));      
        }

        self.canvas.present();
    }

    pub fn draw_statistics(&mut self, model: &Model) {
        self.canvas.clear(); 

        self.draw_background(&model.lines);

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

    fn draw_background(&mut self, lines: &Vec<Line>){
        let carrigeway_width = (CAR_WIDTH * 3 + MARGIN * 6) * 2;
        let field_width = (SCREEN_WIDTH - carrigeway_width) / 2;
        let field_heigth = (SCREEN_HEIGHT - carrigeway_width) / 2;
        let center = Point::new((field_width / 2) as i32, (field_heigth / 2) as i32);
        let src = Rect::new(0, 0, field_width, field_heigth);

        //draw background top-left
        let dst = Rect::new(0, 0, field_width, field_heigth);
        match self.texture_manager.textures.get("top_left") {
            Some(texture) => {
                if let Err(_) = self.canvas
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
                if let Err(_) = self.canvas
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
                if let Err(_) = self.canvas
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
                if let Err(_) = self.canvas
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
        for line in lines {
            line.draw(&mut self.canvas, None);
        }
    }

}

impl Drawable for Line {
    fn draw(&self, canvas: &mut Canvas<Window>, _: Option<&TextureManager>) {
        let (r, g, b) = self.color;
        canvas.set_draw_color(Color::RGB(r, g, b));
        let start = Point::new(self.start.x, self.start.y);
        let end = Point::new(self.end.x, self.end.y);
        canvas.draw_line(start, end).unwrap();
    }
}

impl Drawable for Car {
    fn draw(&self, canvas: &mut Canvas<Window>, texture_manager: Option<&TextureManager>) {

        let x = self.center.x as i32 - CAR_LENGTH_I32 / 2;
        let y = self.center.y as i32 - CAR_WIDTH_I32 / 2;

        match texture_manager {
            Some(texture_manager) => {
                let texture_option = match self.vehicle_type {
                    VehicleType::BlueCar => (texture_manager.textures.get("blue_car"), (0,0,255_u8)),
                    VehicleType::GreenCar => (texture_manager.textures.get("green_car"), (0,255_u8, 0)),
                    VehicleType::RedCar => (texture_manager.textures.get("red_car"),(255_u8, 0, 0))
                };
                match texture_option.0 {
                    Some(texture) => {
                        let src = Rect::new(0, 0, CAR_LENGTH_DEFAULT, CAR_WIDTH_DEFAULT);
                        let dst = Rect::new(x, y, CAR_LENGTH, CAR_WIDTH);
                        let center = Point::new(CAR_LENGTH_I32 / 2, CAR_WIDTH_I32 / 2);
                        if let Err (e) = canvas
                            .copy_ex(
                                &texture,
                                src,
                                dst,
                                self.rotation as f64,
                                center,
                                false,
                                false,
                            ){
                                println!("could not copy texture: {e}"); 
                                canvas.set_draw_color(Color::RGB(texture_option.1.0, texture_option.1.1, texture_option.1.2));
                                let (w, l) = match self.rotation {
                                    0.0 | 180.0 => (CAR_LENGTH, CAR_WIDTH),                           
                                    _ => (CAR_WIDTH, CAR_LENGTH)
                                };
                                if let Err(e) = canvas.fill_rect(Rect::new(x + MARGIN_I32,y,w,l)){
                                    println!("could not draw rect: {e}");
                                }
                            }                       

                    },
                    None => {
                        canvas.set_draw_color(Color::RGB(texture_option.1.0, texture_option.1.1, texture_option.1.2));
                        let (w, l) = match self.rotation {
                            0.0 | 180.0 => (CAR_LENGTH, CAR_WIDTH),                           
                            _ => (CAR_WIDTH, CAR_LENGTH)
                        };

                        if let Err(e) = canvas.fill_rect(Rect::new(x + MARGIN_I32,y,w,l)){
                            println!("could not draw rect: {e}");
                        } 
                    }
                }
            }
            None => {
                //if texture manager is not supplied, use rectangles
                let (r,g,b) = match  self.vehicle_type{
                    VehicleType::BlueCar => (0_u8,0,255),
                    VehicleType::GreenCar => (0,255_u8,0),
                    VehicleType::RedCar => (255,0,0_u8)
                };
                canvas.set_draw_color(Color::RGB(r, g, b));
                let (w, l) = match self.rotation {
                    0.0 | 180.0 => (CAR_LENGTH, CAR_WIDTH),                 
                    _ => (CAR_WIDTH, CAR_LENGTH)
                };

                if let Err(e) = canvas.fill_rect(Rect::new(x + MARGIN_I32,y,w,l)){
                    println!("could not draw rect: {e}");
                } 
            }
        }
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
