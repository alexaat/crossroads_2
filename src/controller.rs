use crate::model::Destination;
use crate::model::VehicleType;
use crate::model::CARDINAL;
use crate::Model;
use crate::View;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
pub struct Controller {
    model: Model,
    view: View,
    pub screen: Screens,
}

impl Controller {
    pub fn new(model: Model, view: View) -> Self {
        let screen = Screens::MAIN;
        Self {
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

    pub fn tick(&mut self) {}

    pub fn get_random_destination() -> Destination {
        let r = rand::thread_rng().gen_range(0..3);
        match r {
            0 => Destination::LEFT,
            1 => Destination::AHEAD,
            _ => Destination::RIGHT,
        }
    }

    pub fn get_random_origin() -> CARDINAL {
        let r = rand::thread_rng().gen_range(0..4);
        match r {
            0 => CARDINAL::WEST,
            1 => CARDINAL::NORTH,
            2 => CARDINAL::EAST,
            _ => CARDINAL::SOUTH,
        }
    }

    pub fn get_random_type() -> VehicleType {
        //random vehicle type
        let r = rand::thread_rng().gen_range(0..3);
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
