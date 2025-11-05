use crate::Model;
use crate::View;
pub struct Controller {
    model: Model,
    view: View,
    pub screen: Screens,
}

impl Controller {}

#[derive(PartialEq)]
pub enum Screens {
    MAIN,
    STATISTICS,
}
