use crate::Model;
use crate::View;
use sdl2::event::Event;
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

    pub fn key_down(&mut self, event: Event) {}

    pub fn tick(&mut self) {}
}

#[derive(PartialEq)]
pub enum Screens {
    MAIN,
    STATISTICS,
}
