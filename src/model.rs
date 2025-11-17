use crate::preferences::*;
use std::f32::consts::PI;
use std::time::Instant;

pub struct Model {
    pub counter: u64,
    pub lines: Vec<Line>,
    pub cars: Vec<Car>,
    pub statistics: Statisics,
}
impl Model {
    pub fn new() -> Self {
        Self {
            counter: 0,
            lines: Self::build_lines(),
            cars: vec![],
            statistics: Statisics::new(),
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

        let car_length = CAR_LENGTH_I32;
        let car_width = CAR_WIDTH_I32;
        let margin = MARGIN_I32;
        let screen_width = SCREEN_WIDTH_I32;
        let screen_heigth = SCREEN_HEIGHT_I32;

        match origin {
            CARDINAL::WEST => {
                x = -car_length;

                match destination {
                    Destination::LEFT => y = TOP_LEFT.y + margin,
                    Destination::AHEAD => y = TOP_LEFT.y + car_width + margin * 3,
                    Destination::RIGHT => y = TOP_LEFT.y + car_width * 2 + margin * 5,
                }
                center_x = x + car_length / 2;
                center_y = y + car_width / 2;
            }

            CARDINAL::EAST => {
                x = screen_width;

                match destination {
                    Destination::RIGHT => y = TOP_LEFT.y + car_width * 3 + margin * 7,
                    Destination::AHEAD => y = TOP_LEFT.y + car_width * 4 + margin * 9,
                    Destination::LEFT => y = TOP_LEFT.y + car_width * 5 + margin * 11,
                }

                center_x = x + car_length / 2;
                center_y = y + car_width / 2;

                // println!();
                // println!("TOP_LEFT.y ${}", TOP_LEFT.y);
                // println!("car_width ${}", car_width);
                // println!("margin ${}", margin);
                // println!("center_y ${}", center_y);
            }

            CARDINAL::NORTH => {
                y = -car_length;

                match destination {
                    Destination::RIGHT => x = TOP_LEFT.x + car_width * 3 + margin * 7,
                    Destination::AHEAD => x = TOP_LEFT.x + car_width * 4 + margin * 9,
                    Destination::LEFT => x = TOP_LEFT.x + car_width * 5 + margin * 11,
                }
                center_x = x + car_width / 2;
                center_y = y + car_length / 2;
            }

            CARDINAL::SOUTH => {
                y = screen_heigth;
                match destination {
                    Destination::RIGHT => x = TOP_LEFT.x + car_width * 2 + margin * 5,
                    Destination::AHEAD => x = TOP_LEFT.x + car_width + margin * 3,
                    Destination::LEFT => x = TOP_LEFT.x + margin,
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

        //println!("spawned center_y: ${center_y}");
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
                    if car.position.x > SCREEN_WIDTH_F32 - CAR_LENGTH_F32 - SEPARATION_DISTANCE_F32
                    {
                        return true;
                    }
                }
                if car.origin == CARDINAL::NORTH {
                    if car.position.y < SEPARATION_DISTANCE_F32 {
                        return true;
                    }
                }
                if car.origin == CARDINAL::SOUTH {
                    if car.position.y > SCREEN_HEIGHT_F32 - CAR_LENGTH_F32 - SEPARATION_DISTANCE_F32
                    {
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

    fn calculate_speed(&mut self, car: &mut Car) -> Option<f32> {
        Some(CAR_MAX_SPEED)
    }

    pub fn update_model(&mut self) {
        self.counter += 1;

        for car in &mut self.cars {
            if car.origin == CARDINAL::WEST {
                //from west turn left
                if car.destination == Destination::LEFT && car.rotation <= -90.0 {
                    car.rotation = -90.0;
                    car.destination = Destination::AHEAD;
                    car.origin = CARDINAL::SOUTH;
                    continue;
                }
                if car.destination == Destination::LEFT && car.center.x >= TOP_LEFT_F32.x {
                    let w = 180.0 * car.speed / (SMALL_RADIUS_F32 * PI);
                    car.rotation -= w;
                    let a = 180.0 + car.rotation;

                    let dx = SMALL_RADIUS_F32 * (a * PI / 180.0).sin();
                    car.center.x = TOP_LEFT_F32.x + dx;

                    let dy = -(SMALL_RADIUS_F32) * (a * PI / 180.0).cos();
                    car.center.y = TOP_LEFT_F32.y + dy;

                    car.odo += SMALL_RADIUS_F32 * w.abs() * PI / 180.0;
                }

                //from west turn right
                if car.destination == Destination::RIGHT && car.rotation >= 90.0 {
                    car.destination = Destination::AHEAD;
                    car.origin = CARDINAL::NORTH;
                    car.rotation = 90.0;
                    continue;
                }
                if car.destination == Destination::RIGHT && car.center.x >= TOP_LEFT_F32.x {
                    let w = 180.0 * car.speed / (PI * BIG_RADIUS_F32);
                    car.rotation += w;
                    let a = car.rotation;

                    let dx = ((a * PI / 180.0).sin()) * BIG_RADIUS_F32;
                    car.center.x = TOP_LEFT_F32.x + dx;

                    let dy = -((a * PI / 180.0).cos()) * BIG_RADIUS_F32;
                    car.center.y = BOTTOM_RIGHT_F32.y + dy;

                    car.odo += BIG_RADIUS_F32 * w.abs() * PI / 180.0;
                }
            }

            if car.origin == CARDINAL::EAST {
                //from east turn left
                //180.0 -> 90.0
                if car.destination == Destination::LEFT && car.rotation <= 90.0 {
                    car.rotation = 90.0;
                    car.destination = Destination::AHEAD;
                    car.origin = CARDINAL::NORTH;
                    continue;
                }

                if car.destination == Destination::LEFT && car.center.x <= BOTTOM_RIGHT_F32.x {
                    let w = 180.0 * car.speed / (PI * SMALL_RADIUS_F32);
                    car.rotation -= w;
                    let a = car.rotation;

                    let dx = SMALL_RADIUS_F32 * (a * PI / 180.0).sin();
                    car.center.x = BOTTOM_RIGHT_F32.x - dx;

                    let dy = SMALL_RADIUS_F32 * (a * PI / 180.0).cos();
                    car.center.y = BOTTOM_RIGHT_F32.y + dy;

                    car.odo += SMALL_RADIUS_F32 * w.abs() * PI / 180.0;
                }

                //from east turn right
                if car.destination == Destination::RIGHT
                    && car.rotation >= 270.0
                    && car.rotation != -90.0
                {
                    car.rotation = -90.0;
                    car.destination = Destination::AHEAD;
                    car.origin = CARDINAL::SOUTH;
                    continue;
                }

                //180.0 -> -90.0
                //180.0 -> 270.0
                if car.destination == Destination::RIGHT && car.center.x <= BOTTOM_RIGHT_F32.x {
                    let w = 180.0 * car.speed / (PI * BIG_RADIUS);
                    car.rotation += w;
                    let a = car.rotation;

                    let dx = BIG_RADIUS_F32 * (a * PI / 180.0).sin();
                    car.center.x = BOTTOM_RIGHT_F32.x + dx;

                    let dy = BIG_RADIUS_F32 * (a * PI / 180.0).cos();
                    car.center.y = TOP_LEFT_F32.y - dy;

                    car.odo += BIG_RADIUS_F32 * w.abs() * PI / 180.0;
                }
            }

            if car.origin == CARDINAL::NORTH {
                //from north turning left
                //LEFT 90.0 -> 0.0
                if car.destination == Destination::LEFT && car.rotation <= 0.0 {
                    car.rotation = 0.0;
                    car.destination = Destination::AHEAD;
                    car.origin = CARDINAL::WEST;
                    continue;
                }

                if car.destination == Destination::LEFT && car.center.y >= TOP_LEFT_F32.y {
                    let w = 180.0 * car.speed / (PI * SMALL_RADIUS_F32);
                    car.rotation -= w;
                    let a = car.rotation;

                    let dx = SMALL_RADIUS_F32 * (a * PI / 180.0).sin();
                    car.center.x = BOTTOM_RIGHT_F32.x - dx;

                    let dy = SMALL_RADIUS_F32 * (a * PI / 180.0).cos();
                    car.center.y = TOP_LEFT_F32.y + dy;

                    car.odo += SMALL_RADIUS_F32 * w.abs() * PI / 180.0;
                }

                //from north turn right
                //RIGHT:  90.0 -> 180.0
                if car.destination == Destination::RIGHT && car.rotation >= 180.0 {
                    car.destination = Destination::AHEAD;
                    car.origin = CARDINAL::EAST;
                    car.rotation = 180.0;
                    continue;
                }

                if car.destination == Destination::RIGHT && car.center.y >= TOP_LEFT_F32.y {
                    let w = 180.0 * car.speed / (PI * BIG_RADIUS);
                    car.rotation += w;
                    let a = car.rotation;

                    let dx = BIG_RADIUS_F32 * (a * PI / 180.0).sin();
                    car.center.x = TOP_LEFT_F32.x + dx;

                    let dy = BIG_RADIUS_F32 * (a * PI / 180.0).cos();
                    car.center.y = TOP_LEFT_F32.y - dy;

                    car.odo += BIG_RADIUS_F32 * w.abs() * PI / 180.0;
                }
            }

            if car.origin == CARDINAL::SOUTH {
                //from south to left
                //      -90.0 -> -180.0
                if car.destination == Destination::LEFT
                    && car.rotation <= -180.0
                    && car.rotation != 180.0
                {
                    car.destination = Destination::AHEAD;
                    car.origin = CARDINAL::EAST;
                    car.rotation = 180.0;
                    continue;
                }

                if car.destination == Destination::LEFT && car.center.y <= BOTTOM_RIGHT_F32.y {
                    let w = 180.0 * car.speed / (PI * SMALL_RADIUS_F32);
                    car.rotation -= w;
                    let a = car.rotation;

                    let dx = SMALL_RADIUS_F32 * (a * PI / 180.0).sin();
                    car.center.x = TOP_LEFT_F32.x - dx;

                    let dy = SMALL_RADIUS_F32 * (a * PI / 180.0).cos();
                    car.center.y = BOTTOM_RIGHT_F32.y + dy;

                    car.odo += SMALL_RADIUS_F32 * w.abs() * PI / 180.0;
                }

                //from south to right
                //RIGHT -90.0 -> 0.0
                if car.destination == Destination::RIGHT && car.rotation >= 0.0 {
                    car.destination = Destination::AHEAD;
                    car.origin = CARDINAL::WEST;
                    car.rotation = 0.0;
                    continue;
                }

                if car.destination == Destination::RIGHT && car.center.y <= BOTTOM_RIGHT_F32.y {
                    let w = 180.0 * car.speed / (PI * BIG_RADIUS_F32);
                    car.rotation += w;
                    let a = car.rotation;

                    let dx = BIG_RADIUS_F32 * (a * PI / 180.0).sin();
                    car.center.x = BOTTOM_RIGHT_F32.x + dx;

                    let dy = BIG_RADIUS_F32 * (a * PI / 180.0).cos();
                    car.center.y = BOTTOM_RIGHT_F32.y - dy;

                    car.odo += BIG_RADIUS_F32 * w.abs() * PI / 180.0;
                }
            }
        }

        // move car
        for car in &mut self.cars {
            if car.origin == CARDINAL::WEST && car.rotation == 0.0 {
                car.position.x += car.speed;
                car.center.x += car.speed;
                car.odo += car.speed;
            }
            if car.origin == CARDINAL::EAST && car.rotation == 180.0 {
                car.position.x -= car.speed;
                car.center.x -= car.speed;
                car.odo += car.speed;
            }
            if car.origin == CARDINAL::NORTH && car.rotation == 90.0 {
                car.position.y += car.speed;
                car.center.y += car.speed;
                car.odo += car.speed;
            }
            if car.origin == CARDINAL::SOUTH && car.rotation == -90.0 {
                car.position.y -= car.speed;
                car.center.y -= car.speed;
                car.odo += car.speed;
            }
        }

        //Remove cars that are no longer visible
        let mut filtered = vec![];
        for (_, car) in self.cars.clone().iter().enumerate() {
            if !(car.center.x as i32 + CAR_LENGTH_I32 * 2 < 0
                || car.center.x as i32 > SCREEN_WIDTH_I32 + CAR_LENGTH_I32 * 2
                || car.center.y as i32 + CAR_LENGTH_I32 * 2 < 0
                || car.center.y as i32 > SCREEN_HEIGHT_I32 + CAR_LENGTH_I32 * 2)
            {
                filtered.push(car.clone());
            }
        }
        self.cars = filtered;
    }

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
                while x <= SCREEN_WIDTH_I32 {
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
                while y <= SCREEN_HEIGHT_I32 {
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
    pub min_time: Option<u128>,
}
impl Statisics {
    pub fn new() -> Self {
        Self {
            number_of_vehicles: 0,
            max_velocity: None,
            min_velocity: None,
            max_time: None,
            min_time: None,
        }
    }
}
