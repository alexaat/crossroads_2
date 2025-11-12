use crate::preferences::*;

pub struct Model {
    pub lines: Vec<Line>,
}
impl Model {
    pub fn new() -> Self {
        Self {
            lines: Self::build_lines(),
        }
    }

    pub fn spawn_car(
        &mut self,
        origin: CARDINAL,
        destination: Destination,
        vehicle_type: VehicleType,
    ) {
    }

    pub fn update_model(&mut self) {}

    fn build_lines() -> Vec<Line> {
        let color = (200, 200, 200);

        let mut lines = vec![];

        let top_left = Point::new(
            (SCREEN_WIDTH as i32 - 6 * (CAR_WIDTH as i32 + 2 * MARGIN as i32)) / 2,
            (SCREEN_HEIGHT as i32 - 6 * (CAR_WIDTH as i32 + 2 * MARGIN as i32)) / 2,
        );
        let bottom_right = Point::new(
            (SCREEN_WIDTH as i32 + 6 * (CAR_WIDTH as i32 + 2 * MARGIN as i32)) / 2,
            (SCREEN_HEIGHT as i32 + 6 * (CAR_WIDTH as i32 + 2 * MARGIN as i32)) / 2,
        );

        //broken lines west side
        for i in 1..6 {
            let x = 0;
            let y = top_left.y + CAR_WIDTH as i32 * i + MARGIN as i32 * i * 2;
            let start = Point::new(x, y);
            let x = top_left.x;

            let end: Point = Point::new(x, y);
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
