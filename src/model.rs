pub struct Model {}
impl Model {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn_car(
        &mut self,
        origin: CARDINAL,
        destination: Destination,
        vehicle_type: VehicleType,
    ) {
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CARDINAL {
    SOUTH,
    NORTH,
    WEST,
    EAST,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Destination {
    LEFT,
    AHEAD,
    RIGHT,
}

#[derive(Debug, Clone)]
pub enum VehicleType {
    RedCar,
    BlueCar,
    GreenCar,
}
