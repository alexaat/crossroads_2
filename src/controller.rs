use crate::model::*;
use crate::View;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
pub struct Controller<'a> {
    model: Model,
    view: View<'a>,
    pub screen: Screens,
}

impl <'a>Controller<'a> {
    pub fn new(model: Model, view: View) -> Controller {
        let screen = Screens::MAIN;
        Controller {
            model,
            view,
            screen,
        }
    }

    pub fn key_down(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => self.model.spawn_car(
                CARDINAL::NORTH,
                Controller::get_random_destination(),
                Controller::get_random_type(),
            ),
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => self.model.spawn_car(
                CARDINAL::SOUTH,
                Controller::get_random_destination(),
                Controller::get_random_type(),
            ),
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => self.model.spawn_car(
                CARDINAL::EAST,
                Controller::get_random_destination(),
                Controller::get_random_type(),
            ),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => self.model.spawn_car(
                CARDINAL::WEST,
                Controller::get_random_destination(),
                Controller::get_random_type(),
            ),
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => self.model.spawn_car(
                Controller::get_random_origin(),
                Controller::get_random_destination(),
                Controller::get_random_type(),
            ),
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        if self.screen == Screens::MAIN {
            self.model.update_model();
            self.view.draw_model(&mut self.model);
        } else if self.screen == Screens::STATISTICS {
            self.view.draw_statistics(&self.model);
        }
    }

    pub fn get_random_destination() -> Destination {
        let r = rand::random_range(0..3);
        match r {
            0 => Destination::LEFT,
            1 => Destination::AHEAD,
            _ => Destination::RIGHT,
        }
    }

    pub fn get_random_origin() -> CARDINAL {
        let r = rand::random_range(0..4);
        match r {
            0 => CARDINAL::WEST,
            1 => CARDINAL::NORTH,
            2 => CARDINAL::EAST,
            _ => CARDINAL::SOUTH,
        }
    }

    pub fn get_random_type() -> VehicleType {
        //random vehicle type
        let r = rand::random_range(0..3);
        match r {
            0 => VehicleType::BlueCar,
            1 => VehicleType::GreenCar,
            _ => VehicleType::RedCar,
        }
    }
}

#[derive(PartialEq)]
pub enum Screens {
    MAIN,
    STATISTICS,
}
