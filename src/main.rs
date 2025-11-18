use controller::Screens;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod model;
use crate::model::Model;
mod view;
use crate::view::View;
mod controller;
use crate::controller::Controller;
mod preferences;
use crate::preferences::*;
use crate::view::{FontManager, TextureManager};
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
mod ui;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("crossroads 2", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new();
    texture_manager.add("top_left", TOP_LEFT_URL, &texture_creator);
    texture_manager.add("blue_car", CAR_URLS[0], &texture_creator);
    texture_manager.add("red_car", CAR_URLS[1], &texture_creator);
    texture_manager.add("green_car", CAR_URLS[2], &texture_creator);

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font_manager = FontManager::new();
    font_manager.add("title", FONT_URL, 32, &ttf_context);
    font_manager.add("body", FONT_URL, 20, &ttf_context);

    let view = View::new(canvas, (0, 0, 0), &texture_manager, &font_manager);
    let model = Model::new();
    let mut controller = Controller::new(model, view);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if controller.screen == Screens::MAIN {
                        controller.screen = Screens::STATISTICS
                    } else if controller.screen == Screens::STATISTICS {
                        break 'running;
                    }
                }
                _ => controller.key_down(event),
            }
        }
        controller.tick();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
