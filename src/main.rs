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
use crate::preferences::SCREEN_HEIGHT;
use crate::preferences::SCREEN_WIDTH;



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

    let view = View::new(canvas, (0, 0, 0));
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
