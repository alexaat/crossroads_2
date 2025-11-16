use crate::preferences::*;
use std::time::Instant;


pub struct Model {
    pub lines: Vec<Line>,
    pub cars: Vec<Car>,
    pub statistics: Statisics,
    pub top_left: Point,
    pub bottom_right: Point,

}
impl Model {
    pub fn new() -> Self {
        Self {
            lines: Self::build_lines(),
            cars: vec![],
            statistics: Statisics::new(),
            top_left: Point::new(
                (SCREEN_WIDTH as i32 - 6 * (CAR_WIDTH as i32 + 2 * MARGIN as i32)) / 2,
                (SCREEN_HEIGHT as i32 - 6 * (CAR_WIDTH as i32 + 2 * MARGIN as i32)) / 2,
            ),
            bottom_right: Point::new(
                (SCREEN_WIDTH as i32 + 6 * (CAR_WIDTH as i32 + 2 * MARGIN as i32)) / 2,
                (SCREEN_HEIGHT as i32 + 6 * (CAR_WIDTH as i32 + 2 * MARGIN as i32)) / 2,
            ),
        }
    }

    pub fn spawn_car(
        &mut self,
        origin: CARDINAL,
        destination: Destination,
        vehicle_type: VehicleType,
    ) {
        if self.is_car_spam(&origin, &destination) {
            return;
        }

        let rotation = match origin {
            CARDINAL::WEST => 0.0,
            CARDINAL::NORTH => 90.0,
            CARDINAL::SOUTH => -90.0,
            _ => 180.0,
        };

        let x;
        let y;

        let center_x;
        let center_y;

        let car_length = CAR_LENGTH as i32;
        let car_width = CAR_WIDTH as i32;
        let margin = MARGIN as i32;
        let screen_width = SCREEN_WIDTH as i32;
        let screen_heigth = SCREEN_HEIGHT as i32;

        match origin {
            CARDINAL::WEST => {
                x = -car_length;

                match destination {
                    Destination::LEFT => y = self.top_left.y + margin,
                    Destination::AHEAD => y = self.top_left.y + car_width + margin * 3,
                    Destination::RIGHT => y = self.top_left.y + car_width * 2 + margin * 5,
                }
                center_x = x + car_length / 2;
                center_y = y + car_width / 2;
            }

            CARDINAL::EAST => {
                x = screen_width;

                match destination {
                    Destination::RIGHT => y = self.top_left.y + car_width * 3 + margin * 7,
                    Destination::AHEAD => y = self.top_left.y + car_width * 4 + margin * 9,
                    Destination::LEFT => y = self.top_left.y + car_width * 5 + margin * 11,
                }
                center_x = x + car_length / 2;
                center_y = y + car_width / 2;
            }

            CARDINAL::NORTH => {
                y = -car_length;

                match destination {
                    Destination::RIGHT => x = self.top_left.x + car_width * 3 + margin * 7,
                    Destination::AHEAD => x = self.top_left.x + car_width * 4 + margin * 9,
                    Destination::LEFT => x = self.top_left.x + car_width * 5 + margin * 11,
                }
                center_x = x + car_width / 2;
                center_y = y + car_length / 2;
            }

            CARDINAL::SOUTH => {
                y = screen_heigth;
                match destination {
                    Destination::RIGHT => x = self.top_left.x + car_width * 2 + margin * 5,
                    Destination::AHEAD => x = self.top_left.x + car_width + margin * 3,
                    Destination::LEFT => x = self.top_left.x + margin,
                }
                center_x = x + car_width / 2;
                center_y = y + car_length / 2;
            }
        }

        let id = self.generate_car_id();

        let mut car = Car::new(
            id,
            origin,
            destination,
            PointF::new(x as f32, y as f32),
            rotation,
            PointF::new(center_x as f32, center_y as f32),
        );
        let speed_option = self.calculate_speed(&mut car);
        if let Some(speed) = speed_option {
            car.speed = speed;
            car.vehicle_type = vehicle_type;
            self.cars.push(car.clone());
            //
            //number of cars
            self.statistics.number_of_vehicles += 1;
            //
            //max velocity
            if let Some(max) = self.statistics.max_velocity {
                if speed > max {
                    self.statistics.max_velocity = Some(speed);
                }
            } else {
                self.statistics.max_velocity = Some(speed);
            }
            //
            //min velocity
            if let Some(min) = self.statistics.min_velocity {
                if speed < min {
                    self.statistics.min_velocity = Some(speed);
                }
            } else {
                self.statistics.min_velocity = Some(speed);
            }
        }
    }

    fn is_car_spam(&mut self, origin: &CARDINAL, destination: &Destination) -> bool {
        for car in &self.cars {
            if car.origin == *origin && car.destination == *destination {
                if car.origin == CARDINAL::WEST {
                    if car.position.x < SEPARATION_DISTANCE_F32 {
                        return true;
                    }
                }
                if car.origin == CARDINAL::EAST {
                    if car.position.x > SCREEN_WIDTH_F32 - CAR_LENGTH_F32 - SEPARATION_DISTANCE_F32 {
                        return true;
                    }
                }
                if car.origin == CARDINAL::NORTH {
                    if car.position.y < SEPARATION_DISTANCE_F32 {
                        return true;
                    }
                }
                if car.origin == CARDINAL::SOUTH {
                    if car.position.y > SCREEN_HEIGHT_F32 - CAR_LENGTH_F32 - SEPARATION_DISTANCE_F32 {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn generate_car_id(&mut self) -> u64 {
        if self.cars.len() == 0 {
            return 1;
        }
        self.cars.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
        let id = self.cars[self.cars.len() - 1].id + 1;
        return id;
    }

    fn calculate_speed(&mut self, car: &mut Car) -> Option<f32>{
        Some(CAR_MAX_SPEED)
    }

    pub fn update_model(&mut self) {}

    fn build_lines() -> Vec<Line> {
        let color = (200, 200, 200);

        let mut lines = vec![];

        //broken lines west side
        for i in 1..6 {
            let x = 0;
            let y = TOP_LEFT.y + CAR_WIDTH_I32 * i + MARGIN_I32 * i * 2;
            let start = Point::new(x, y);
            let x = TOP_LEFT.x;

            let end: Point = Point::new(x, y);
            if i == 3 {
                let line = Line::new(start, end, color);
                lines.push(line.clone());
            } else {
                Self::build_broken_lines(&mut lines, start, end);
            }
        }

        //broken lines east side
        for i in 1..6 {
            let x = BOTTOM_RIGHT.x;
            let y = TOP_LEFT.y + CAR_WIDTH_I32 * i + MARGIN_I32 * i * 2;
            let end = Point::new(x, y);
            let x = SCREEN_WIDTH_I32;
            let start: Point = Point::new(x, y);
            if i == 3 {
                let line = Line::new(start, end, color);
                lines.push(line.clone());
            } else {
                Self::build_broken_lines(&mut lines, start, end);
            }
        }

        //broken lines north
        for i in 1..6 {
            let x = TOP_LEFT.x + CAR_WIDTH_I32 * i + MARGIN_I32 * i * 2;
            let y = 0;
            let start = Point::new(x, y);
            let y = TOP_LEFT.y;
            let end: Point = Point::new(x, y);
            if i == 3 {
                let line = Line::new(start, end, color);
                lines.push(line.clone());
            } else {
                Self::build_broken_lines(&mut lines, start, end);
            }
        }

        //broken lines south
        for i in 1..6 {
            let x = TOP_LEFT.x + CAR_WIDTH_I32 * i + MARGIN_I32 * i * 2;
            let y = BOTTOM_RIGHT.y;
            let end = Point::new(x, y);
            let y = SCREEN_HEIGHT_I32;
            let start: Point = Point::new(x, y);
            if i == 3 {
                let line = Line::new(start, end, color);
                lines.push(line.clone());
            } else {
                Self::build_broken_lines(&mut lines, start, end);
            }
        }

        lines
    }

    fn build_broken_lines(lines: &mut Vec<Line>, start: Point, end: Point) {
        let color = (90, 90, 90);
        //horizontal
        if start.y == end.y {
            if end.x > start.x {
                let mut x = end.x;
                while x >= 0 {
                    let line = Line::new(
                        Point::new(x, end.y),
                        Point::new(x - LINE_LENGHT, start.y),
                        color,
                    );
                    lines.push(line.clone());
                    x = x - LINE_LENGHT - LINE_GAP;
                }
            } else {
                let mut x = end.x;
                while x <= SCREEN_WIDTH as i32 {
                    let line = Line::new(
                        Point::new(x, end.y),
                        Point::new(x + LINE_LENGHT, start.y),
                        color,
                    );
                    lines.push(line.clone());
                    x = x + LINE_LENGHT + LINE_GAP;
                }
            }
        }
        if start.x == end.x {
            if end.y > start.y {
                let mut y = end.y;
                while y > 0 {
                    let line = Line::new(
                        Point::new(end.x, y),
                        Point::new(start.x, y - LINE_LENGHT),
                        color,
                    );
                    lines.push(line.clone());
                    y = y - LINE_LENGHT - LINE_GAP;
                }
            } else {
                let mut y = end.y;
                while y <= SCREEN_HEIGHT as i32 {
                    let line = Line::new(
                        Point::new(end.x, y),
                        Point::new(start.x, y + LINE_LENGHT),
                        color,
                    );
                    lines.push(line.clone());
                    y = y + LINE_LENGHT + LINE_GAP;
                }
            }
        }
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

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: (u8, u8, u8),
}
impl Line {
    pub fn new(start: Point, end: Point, color: (u8, u8, u8)) -> Self {
        Self { start, end, color }
    }
}

#[derive(Debug, Clone)]
pub struct Car {
    pub id: u64,
    pub origin: CARDINAL,
    pub destination: Destination,
    pub position: PointF,
    pub rotation: f32,
    pub speed: f32,
    pub center: PointF,
    pub odo: f32,
    pub timer: Instant,
    pub vehicle_type: VehicleType,
}
impl Car {
    pub fn new(
        id: u64,
        origin: CARDINAL,
        destination: Destination,
        position: PointF,
        rotation: f32,
        center: PointF,
    ) -> Self {
        let speed = CAR_MAX_SPEED;
        let odo: f32 = -(CAR_LENGTH as f32 * 0.5);
        let timer: Instant = Instant::now();
        let vehicle_type: VehicleType = VehicleType::BlueCar;

        Self {
            id,
            origin,
            destination,
            position,
            rotation,
            speed,
            center,
            odo,
            timer,
            vehicle_type,
        }
    }

}

#[derive(Debug, Clone)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}
impl PointF {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Statisics {
    pub number_of_vehicles: u32,
    pub max_velocity: Option<f32>,
    pub min_velocity: Option<f32>,
    pub max_time: Option<u128>,
    pub min_time: Option<u128>
}
impl Statisics {
    pub fn new() -> Self {
        Self {
            number_of_vehicles: 0,
            max_velocity: None,
            min_velocity: None,
            max_time: None,
            min_time: None
        }
    }
}


