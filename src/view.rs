use crate::model::Line;
use crate::Model;
use crate::preferences::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use super::*;
use std::collections::HashMap;

pub struct View<'a>  {
    canvas: Canvas<Window>,
    bg_color: (u8, u8, u8),
    texture_manager: &'a TextureManager<'a>
}

impl<'a> View<'a> {
    pub fn new(canvas: Canvas<Window>, bg_color: (u8, u8, u8), texture_manager: &'a TextureManager) -> View<'a> {
        
        //let scene = Scene::new(&canvas);
        View { canvas, bg_color,  texture_manager }
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
        match self.texture_manager.textures.get("top_left"){
            Some(texture) => {
                if let Err(_) = self.canvas.copy_ex( texture, src, dst, 0.0, center, false, false){
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    self.canvas.fill_rect(dst).unwrap();                   
                }
            }, None => {
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

        match self.texture_manager.textures.get("top_left"){
            Some(texture) => {
                if let Err(_) = self.canvas.copy_ex( texture, src, dst, 0.0, center, false, true){
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    self.canvas.fill_rect(dst).unwrap();                   
                }
            }, None => {
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

        match self.texture_manager.textures.get("top_left"){
            Some(texture) => {
                if let Err(_) = self.canvas.copy_ex( texture, src, dst, 0.0, center, true, false){
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    self.canvas.fill_rect(dst).unwrap();
                }
            }, None => {
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

        match self.texture_manager.textures.get("top_left"){
            Some(texture) => {
                if let Err(_) = self.canvas.copy_ex(texture, src, dst, 0.0, center, true, true){
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    self.canvas.fill_rect(dst).unwrap();
                }
            }, None => {
                self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                self.canvas.fill_rect(dst).unwrap();              
            }
        }
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


pub struct TextureManager<'a> {
    pub textures: HashMap<String, Texture<'a>>

}
impl<'a> TextureManager<'a>{

    pub fn new() -> TextureManager<'a>{
        TextureManager {textures: HashMap::new()}
    }

    pub fn add(&mut self, name: &str, path: &str, texture_creator: &'a TextureCreator<WindowContext>){
        match texture_creator.load_texture(path){
            Ok(texture) => {
                self.textures.insert(name.to_string(), texture);
            },
            Err(e) => println!("cannot load texture: {:?}", e)                 
        }       
    }
}

/*
pub struct TextureManager<'a> {
    pub textures: HashMap<String, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        name: &str,
        path: &str,
        texture_creator:  &'a TextureCreator<WindowContext>,
    ) {
        let texture = texture_creator.load_texture(path).unwrap();
        self.textures.insert(name.to_string(), texture);
    }

    pub fn get(&self, name: &str) -> &Texture<'a> {
        self.textures.get(name).unwrap()
    }
}
*/


