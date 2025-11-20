use crate::preferences::*;
use std::f32::consts::PI;
use std::time::Instant;
use crate::math::*;

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
        let w = CAR_WIDTH_F32;
        let m = MARGIN_F32;
        let l = CAR_LENGTH_F32;
        let s_w = SCREEN_WIDTH_F32;
        let s_h = SCREEN_HEIGHT_F32;
        let mut car_in_front: Option<Car> = None;
        let mut speed_intervals: Vec<Interval> = vec![];
        //
        //from WEST
        if car.origin == CARDINAL::WEST {
            let dist = TOP_LEFT_F32.x + CAR_LENGTH_F32 * 0.5;
            //
            //from WEST LEFT
            if car.destination == Destination::LEFT {
                return Some(dist / (MIN_TIME * 2.0));
            }
            //from WEST AHEAD
            if car.destination == Destination::AHEAD {
                for c in self.cars.clone() {
                    //
                    //from NORTH
                    if c.origin == CARDINAL::NORTH && c.center.y < BOTTOM_RIGHT_F32.y {
                        //
                        //from NORTH AHEAD
                        if c.destination == Destination::AHEAD {
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.y + w + 2.0 * m - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = TOP_LEFT_F32.y + 2.0 * w + 4.0 * m + 0.5 * l - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + 4.0 * w + 8.0 * m - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + 5.0 * w + 10.0 * m + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                        //
                        //from NORTH RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_two();
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.y + calc.smaller_arc - c.odo;
                            //exit danger zone
                            let d2 = TOP_LEFT_F32.y + calc.bigger_arc - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_straight - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_straight + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from EAST
                    if c.origin == CARDINAL::EAST && c.center.x > TOP_LEFT_F32.x {
                        //
                        //from EAST RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_one();
                            //enter danger zone
                            let d1 = s_w - BOTTOM_RIGHT_F32.x + calc.smaller_arc - c.odo;
                            //exit danger zone
                            let d2 = s_w - BOTTOM_RIGHT_F32.x + calc.bigger_arc - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = BOTTOM_RIGHT_F32.x
                                - calc.bigger_straight
                                - 0.5 * l
                                - SPACE_GAP
                                - car.odo;
                            let dd = BOTTOM_RIGHT_F32.x - calc.smaller_straight
                                + 0.5 * l
                                + SPACE_GAP
                                - car.odo;
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from SOUTH
                    if c.origin == CARDINAL::SOUTH && c.center.y > TOP_LEFT_F32.y {
                        //
                        //from SOUTH AHEAD
                        if c.destination == Destination::AHEAD {
                            //enter danger zone
                            let d1 = s_h - BOTTOM_RIGHT_F32.y + 4.0 * w + 8.0 * m
                                - 0.5 * l
                                - c.odo;
                            //exit danger zone
                            let d2 =
                                s_h - BOTTOM_RIGHT_F32.y + 5.0 * w + 10.0 * m + 0.5 * l
                                    - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + w + 2.0 * m - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + 2.0 * w + 4.0 * m + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //same origin (from WEST)
                    if c.origin == CARDINAL::WEST
                        && c.destination == Destination::AHEAD
                        && c.center.x < BOTTOM_RIGHT_F32.x
                    {
                        //get car in front
                        if let Some(ref value) = car_in_front {
                            if value.center.x > c.center.x {
                                car_in_front = Some(c.clone());
                            }
                        } else {
                            if c.center.x > car.center.x {
                                car_in_front = Some(c.clone());
                            }
                        }
                    }
                }
            }
            //
            //from WEST RIGHT
            if car.destination == Destination::RIGHT {
                for c in self.cars.clone() {
                    //
                    //from NORTH
                    if c.origin == CARDINAL::NORTH && c.center.y < BOTTOM_RIGHT_F32.y {
                        //from NORTH RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_three();
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.y + calc.arc_two - c.odo;
                            //exit danger zone
                            let d2 =
                                TOP_LEFT_F32.y + 0.5 * PI * BIG_RADIUS + 0.5 * l - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + calc.arc_one + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from EAST
                    if c.origin == CARDINAL::EAST && c.center.x > TOP_LEFT_F32.x {
                        //
                        //from EAST AHEAD
                        if c.destination == Destination::AHEAD {
                            let arc_calc = get_arc_len_with_straigh_path_one();
                            let d1 = s_w
                                - TOP_LEFT_F32.x
                                - arc_calc.bigger_straight
                                - 0.5 * l
                                - c.odo;
                            let d2 = s_w - TOP_LEFT_F32.x - arc_calc.smaller_straight
                                + 0.5 * l
                                - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            // main car
                            let d = dist + arc_calc.smaller_arc - SPACE_GAP - car.odo;
                            let dd = dist + arc_calc.bigger_arc + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from SOUTH
                    if c.origin == CARDINAL::SOUTH && c.center.y > TOP_LEFT_F32.y {
                        //
                        //from SOUTH AHEAD
                        if c.destination == Destination::AHEAD {
                            let arc_calc = get_arc_len_with_straigh_path_two();
                            //enter danger zone
                            let d1 = s_h - BOTTOM_RIGHT_F32.y + arc_calc.smaller_straight
                                - 0.5 * l
                                - c.odo;
                            //exit danger zone
                            let d2 = s_h - BOTTOM_RIGHT_F32.y
                                + arc_calc.bigger_straight
                                + 0.5 * l
                                - c.odo;

                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + arc_calc.smaller_arc - SPACE_GAP - car.odo;
                            let dd = dist + arc_calc.bigger_arc + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                        //
                        //from SOUTH RIGHT
                        if c.destination == Destination::RIGHT {
                            let arc_calc = get_arc_len_with_straigh_path_three();
                            //enter danger zone
                            let d1 = s_h - BOTTOM_RIGHT_F32.y - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = s_h - BOTTOM_RIGHT_F32.y + arc_calc.arc_one - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + arc_calc.arc_two - SPACE_GAP - car.odo;
                            let dd = dist + 0.5 * PI * BIG_RADIUS + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //same origin (from WEST)
                    if c.origin == CARDINAL::WEST && c.destination == Destination::RIGHT {
                        //get car in front
                        if let Some(ref value) = car_in_front {
                            if value.center.x > c.center.x {
                                car_in_front = Some(c.clone());
                            }
                        } else {
                            if c.center.x > car.center.x {
                                car_in_front = Some(c.clone());
                            }
                        }
                    }
                }
            }
            //
            //adjust for car in front
            let mut max_speed: Option<f32> = None;
            if let Some(ref c_front) = car_in_front {
                //time for car in front to get to safe area
                let d = BOTTOM_RIGHT_F32.x - c_front.center.x;
                let t_front = d / c_front.speed;

                //speed that is needed to reach car in front
                let dd = c_front.center.x
                    - car.center.x
                    - SEPARATION_DISTANCE as f32
                    - CAR_LENGTH_F32;
                let v0 = dd / t_front;
                let v = v0 + c_front.speed;
                max_speed = Some(v);
            }
            //calculate speed based on time windows
            return get_speed_loop(dist, speed_intervals, max_speed);
        }
        //
        //from EAST
        if car.origin == CARDINAL::EAST {
            let dist = s_w - BOTTOM_RIGHT_F32.x + CAR_LENGTH_F32 * 0.5;
            //
            //from EAST LEFT
            if car.destination == Destination::LEFT {
                return Some(dist / (MIN_TIME * 2.0));
            }
            //
            //from EAST AHEAD
            if car.destination == Destination::AHEAD {
                for c in self.cars.clone() {
                    //
                    //from WEST
                    if c.origin == CARDINAL::WEST && c.center.x < BOTTOM_RIGHT_F32.x {
                        //from WEST RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc_one = get_arc_len_with_straigh_path_one();
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.x + calc_one.smaller_arc - c.odo;
                            //exit danger zone
                            let d2 = TOP_LEFT_F32.x + calc_one.bigger_arc - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = s_w
                                - TOP_LEFT_F32.x
                                - calc_one.bigger_straight
                                - 0.5 * l
                                - SPACE_GAP
                                - car.odo;
                            let dd = s_w - TOP_LEFT_F32.x - calc_one.smaller_straight
                                + 0.5 * l
                                + SPACE_GAP
                                - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from NORTH
                    if c.origin == CARDINAL::NORTH && c.center.y < BOTTOM_RIGHT_F32.y {
                        //
                        //from NORTH AHEAD
                        if c.destination == Destination::AHEAD {
                            //enter danger zone
                            let d1 =
                                BOTTOM_RIGHT_F32.y - 2.0 * w - 4.0 * m - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = BOTTOM_RIGHT_F32.y - w - 2.0 * m + 0.5 * l - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + w + 2.0 * m - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + 3.0 * w + 6.0 * m + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from SOUTH
                    if c.origin == CARDINAL::SOUTH && c.center.y > TOP_LEFT_F32.y {
                        //
                        //from SOUTH AHEAD
                        if c.destination == Destination::AHEAD {
                            //enter danger zone
                            let d1 =
                                s_h - BOTTOM_RIGHT_F32.y + w + 2.0 * m - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = s_h - BOTTOM_RIGHT_F32.y + 2.0 * w + 4.0 * m + 0.5 * l
                                - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + 4.0 * w + 8.0 * m - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + 5.0 * w + 10.0 * m + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                        //
                        //from SOUTH RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_two();
                            //enter danger zone
                            let d1 = s_h - BOTTOM_RIGHT_F32.y + calc.smaller_arc - c.odo;
                            //exit danger zone
                            let d2 = s_h - BOTTOM_RIGHT_F32.y + calc.bigger_arc - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_straight - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_straight + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //(same origin) form EAST
                    if c.origin == CARDINAL::EAST
                        && c.destination == Destination::AHEAD
                        && c.center.x > TOP_LEFT_F32.x
                    {
                        //get car in front
                        if let Some(ref value) = car_in_front {
                            if value.center.x < c.center.x {
                                car_in_front = Some(c.clone());
                            }
                        } else {
                            if c.center.x < car.center.x {
                                car_in_front = Some(c.clone());
                            }
                        }
                    }
                }
            }
            //
            //from EAST RIGHT
            if car.destination == Destination::RIGHT {
                for c in self.cars.clone() {
                    //
                    //from WEST
                    if c.origin == CARDINAL::WEST && c.center.x < BOTTOM_RIGHT_F32.x {
                        //from WEST AHEAD
                        if c.destination == Destination::AHEAD {
                            let calc = get_arc_len_with_straigh_path_one();
                            //enter danger zone
                            let d1 =
                                BOTTOM_RIGHT_F32.x - calc.bigger_straight - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = BOTTOM_RIGHT_F32.x - calc.smaller_straight + 0.5 * l
                                - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_arc - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_arc + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from NORTH
                    if c.origin == CARDINAL::NORTH && c.center.y < BOTTOM_RIGHT_F32.y {
                        //
                        //from NORTH AHEAD
                        if c.destination == Destination::AHEAD {
                            let calc = get_arc_len_with_straigh_path_two();
                            //enter danger zone
                            let d1 =
                                TOP_LEFT_F32.y + calc.smaller_straight - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 =
                                TOP_LEFT_F32.y + calc.bigger_straight + 0.5 * l - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_arc - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_arc + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                        //
                        //from NORTH RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_three();
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.y - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = TOP_LEFT_F32.y + calc.arc_one - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.arc_two - SPACE_GAP - car.odo;
                            let dd = dist + 0.5 * PI * BIG_RADIUS + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from SOUTH
                    if c.origin == CARDINAL::SOUTH && c.center.y > TOP_LEFT_F32.y {
                        //
                        //from SOUTH RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_three();
                            //enter danger zone
                            let d1 = s_h - BOTTOM_RIGHT_F32.y + calc.arc_two - c.odo;
                            //exit danger zone
                            let d2 =
                                s_h - BOTTOM_RIGHT_F32.y + 0.5 * PI * BIG_RADIUS + 0.5 * l
                                    - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + calc.arc_one + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //same origin (from EAST)
                    if c.origin == CARDINAL::EAST
                        && c.destination == Destination::RIGHT
                        && c.center.x > (TOP_LEFT_F32.x + 2.5 * w + 5.0 * m)
                    {
                        //get car in front
                        if let Some(ref value) = car_in_front {
                            if value.center.x < c.center.x {
                                car_in_front = Some(c.clone());
                            }
                        } else {
                            if c.center.x < car.center.x {
                                car_in_front = Some(c.clone());
                            }
                        }
                    }
                }
            }
            //
            //adjust for car in front
            let mut max_speed: Option<f32> = None;
            if let Some(ref c_front) = car_in_front {
                //time for car in front to get to safe area
                let d = c_front.center.x - TOP_LEFT_F32.x;
                let t_front = d / c_front.speed;

                //speed that is needed to reach car in front
                let dd = car.center.x
                    - c_front.center.x
                    - SEPARATION_DISTANCE as f32
                    - CAR_LENGTH_F32;
                let v0 = dd / t_front;
                let v = v0 + c_front.speed;
                max_speed = Some(v);
            }
            //calculate speed based on time windows
            return get_speed_loop(dist, speed_intervals, max_speed);
        }
        //
        //from NORTH
        if car.origin == CARDINAL::NORTH {
            let dist = TOP_LEFT_F32.y + CAR_LENGTH_F32 * 0.5;
            //
            //from NORTH LEFT
            if car.destination == Destination::LEFT {
                return Some(dist / (MIN_TIME * 2.0));
            }
            //
            //from NORTH AHEAD
            if car.destination == Destination::AHEAD {
                for c in self.cars.clone() {
                    //
                    //from WEST
                    if c.origin == CARDINAL::WEST && c.center.x < BOTTOM_RIGHT_F32.x {
                        //from WEST AHEAD
                        if c.destination == Destination::AHEAD {
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.x + 4.0 * w + 8.0 * m - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = BOTTOM_RIGHT_F32.x - w - 2.0 * m + 0.5 * l - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + w + 2.0 * m - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + 2.0 * w + 4.0 * m + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from EAST
                    if c.origin == CARDINAL::EAST && c.center.x > TOP_LEFT_F32.x {
                        //
                        //from EAST AHEAD
                        if c.destination == Destination::AHEAD {
                            let d1 =
                                s_w - BOTTOM_RIGHT_F32.x + w + 2.0 * m - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = s_w - BOTTOM_RIGHT_F32.x + 2.0 * w + 4.0 * m + 0.5 * l
                                - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + 4.0 * w + 8.0 * m - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + 5.0 * w + 10.0 * m + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                        //
                        //from EAST RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_two();
                            //enter danger zone
                            let d1 = s_w - BOTTOM_RIGHT_F32.x + calc.smaller_arc - c.odo;
                            //exit danger zone
                            let d2 = s_w - BOTTOM_RIGHT_F32.x + calc.bigger_arc - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_straight - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_straight + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from SOUTH
                    if c.origin == CARDINAL::SOUTH && c.center.y > TOP_LEFT_F32.y {
                        //
                        //from SOUTH RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_one();
                            //enter danger zone
                            let d1 = s_h - BOTTOM_RIGHT_F32.y + calc.smaller_arc - c.odo;
                            //exit danger zone
                            let d2 = s_h - BOTTOM_RIGHT_F32.y + calc.bigger_arc - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = BOTTOM_RIGHT_F32.y
                                - calc.bigger_straight
                                - 0.5 * l
                                - SPACE_GAP
                                - car.odo;
                            let dd = BOTTOM_RIGHT_F32.y - calc.smaller_straight
                                + 0.5 * l
                                + SPACE_GAP
                                - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //same origin (from NORTH)
                    if c.origin == CARDINAL::NORTH
                        && c.destination == Destination::AHEAD
                        && c.center.y < BOTTOM_RIGHT_F32.y
                    {
                        //get car in front
                        if let Some(ref value) = car_in_front {
                            if value.center.y > c.center.y {
                                car_in_front = Some(c.clone());
                            }
                        } else {
                            if c.center.y > car.center.y {
                                car_in_front = Some(c.clone());
                            }
                        }
                    }
                }
            }
            //
            //from NORTH RIGHT
            if car.destination == Destination::RIGHT {
                for c in self.cars.clone() {
                    //
                    //from WEST
                    if c.origin == CARDINAL::WEST && c.center.x < BOTTOM_RIGHT_F32.x {
                        //
                        //from WEST AHEAD
                        if c.destination == Destination::AHEAD {
                            let calc = get_arc_len_with_straigh_path_two();
                            //enter danger zone
                            let d1 =
                                TOP_LEFT_F32.x + calc.smaller_straight - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 =
                                TOP_LEFT_F32.x + calc.bigger_straight + 0.5 * l - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_arc - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_arc + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                        //
                        //from WEST RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_three();
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.x - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = TOP_LEFT_F32.x + calc.arc_one - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.arc_two - SPACE_GAP - car.odo;
                            let dd = dist + 0.5 * PI * BIG_RADIUS + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from EAST
                    if c.origin == CARDINAL::EAST && c.center.x > TOP_LEFT_F32.x {
                        //
                        //from EAST RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_three();
                            //enter danger zone
                            let d1 = s_w - BOTTOM_RIGHT_F32.x + calc.arc_two - c.odo;
                            //exit danger zone
                            let d2 =
                                s_w - BOTTOM_RIGHT_F32.x + 0.5 * PI * BIG_RADIUS + 0.5 * l
                                    - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + calc.arc_one + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from SOUTH
                    if c.origin == CARDINAL::SOUTH && c.center.y > TOP_LEFT_F32.y {
                        //
                        //from SOUTH AHEAD
                        if c.destination == Destination::AHEAD {
                            let calc = get_arc_len_with_straigh_path_one();
                            //enter danger zone
                            let d1 = s_h
                                - TOP_LEFT_F32.y
                                - calc.bigger_straight
                                - 0.5 * l
                                - c.odo;
                            //exit danger zone
                            let d2 = s_h - TOP_LEFT_F32.y - calc.smaller_straight + 0.5 * l
                                - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_arc - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_arc + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //same origin (from NORTH)
                    if c.origin == CARDINAL::NORTH
                        && c.destination == Destination::RIGHT
                        && c.center.y < (TOP_LEFT_F32.y + 3.5 * w + 7.0 * m)
                    {
                        //get car in front
                        if let Some(ref value) = car_in_front {
                            if value.center.y > c.center.y {
                                car_in_front = Some(c.clone());
                            }
                        } else {
                            if c.center.y > car.center.y {
                                car_in_front = Some(c.clone());
                            }
                        }
                    }
                }
            }
            //adjust for car in front
            let mut max_speed: Option<f32> = None;
            if let Some(ref c_front) = car_in_front {
                //time for car in front to get to safe area
                let d = BOTTOM_RIGHT_F32.y - c_front.center.y;
                let t_front = d / c_front.speed;

                //speed that is needed to reach car in front
                let dd = c_front.center.y
                    - car.center.y
                    - SEPARATION_DISTANCE as f32
                    - CAR_LENGTH_F32;
                let v0 = dd / t_front;
                let v = v0 + c_front.speed;
                max_speed = Some(v);
            }

            //calculate speed based on time windows
            return get_speed_loop(dist, speed_intervals, max_speed);
        }
        //
        //from SOUTH
        if car.origin == CARDINAL::SOUTH {
            let dist = s_h - BOTTOM_RIGHT_F32.y + CAR_LENGTH_F32 * 0.5;
            //
            //from SOUTH LEFT
            if car.destination == Destination::LEFT {
                return Some(dist / (MIN_TIME * 2.0));
            }
            //
            //from SOUTH AHEAD
            if car.destination == Destination::AHEAD {
                for c in self.cars.clone() {
                    //
                    //from WEST
                    if c.origin == CARDINAL::WEST && c.center.x < BOTTOM_RIGHT_F32.x {
                        //
                        //from WEST AHEAD
                        if c.destination == Destination::AHEAD {
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.x + w + 2.0 * m - 0.5 * l - c.odo;

                            //exit danger zone
                            let d2 = TOP_LEFT_F32.x + 2.0 * w + 4.0 * m + 0.5 * l - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + 3.0 * w + 6.0 * m + 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + 5.0 * w + 10.0 * m + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                        //
                        //from WEST RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_two();
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.x + calc.smaller_arc - c.odo;
                            //exit danger zone
                            let d2 = TOP_LEFT_F32.x + calc.bigger_arc - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_straight - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_straight + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from NORTH
                    if c.origin == CARDINAL::NORTH && c.center.y < BOTTOM_RIGHT_F32.y {
                        //
                        //from NORTH RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_one();
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.y + calc.smaller_arc - c.odo;
                            //exit danger zone
                            let d2 = TOP_LEFT_F32.y + calc.bigger_arc - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = s_h
                                - TOP_LEFT_F32.y
                                - calc.bigger_straight
                                - 0.5 * l
                                - SPACE_GAP
                                - car.odo;
                            let dd = s_h - TOP_LEFT_F32.y - calc.smaller_straight
                                + 0.5 * l
                                + SPACE_GAP
                                - car.odo;

                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from EAST
                    if c.origin == CARDINAL::EAST && c.center.x > TOP_LEFT_F32.x {
                        //
                        //from EAST AHEAD
                        if c.destination == Destination::AHEAD {
                            //enter danger zone
                            let d1 = s_w - BOTTOM_RIGHT_F32.x + 4.0 * w + 8.0 * m
                                - 0.5 * l
                                - c.odo;
                            //exit danger zone
                            let d2 =
                                s_w - BOTTOM_RIGHT_F32.x + 5.0 * w + 10.0 * m + 0.5 * l
                                    - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + w + 2.0 * m - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + 2.0 * w + 4.0 * m + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //same origin (FROM SOUTH)
                    if c.origin == CARDINAL::SOUTH
                        && c.destination == Destination::AHEAD
                        && c.center.y > TOP_LEFT_F32.y
                    {
                        //get car in front
                        if let Some(ref value) = car_in_front {
                            if value.center.y < c.center.y {
                                car_in_front = Some(c.clone());
                            }
                        } else {
                            if c.center.y < car.center.y {
                                car_in_front = Some(c.clone());
                            }
                        }
                    }
                }
            }
            //
            //from SOUTH RIGHT
            if car.destination == Destination::RIGHT {
                for c in self.cars.clone() {
                    //
                    //from WEST
                    if c.origin == CARDINAL::WEST && c.center.x < BOTTOM_RIGHT_F32.x {
                        //
                        //from WEST RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_three();
                            //enter danger zone
                            let d1 = TOP_LEFT_F32.x + calc.arc_two - c.odo;
                            //exit danger zone
                            let d2 =
                                TOP_LEFT_F32.x + 0.5 * PI * BIG_RADIUS + 0.5 * l - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist - 0.5 * l - SPACE_GAP - car.odo;
                            let dd = dist + calc.arc_one + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from NORTH
                    if c.origin == CARDINAL::NORTH && c.center.y < BOTTOM_RIGHT_F32.y {
                        //
                        //from NORTH AHEAD
                        if c.destination == Destination::AHEAD {
                            let calc = get_arc_len_with_straigh_path_one();
                            //enter danger zone
                            let d1 =
                                BOTTOM_RIGHT_F32.y - calc.bigger_straight - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = BOTTOM_RIGHT_F32.y - calc.smaller_straight + 0.5 * l
                                - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_arc - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_arc + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //
                    //from EAST
                    if c.origin == CARDINAL::EAST && c.center.x > TOP_LEFT_F32.x {
                        //
                        //from EAST AHEAD
                        if c.destination == Destination::AHEAD {
                            let calc = get_arc_len_with_straigh_path_two();
                            //enter danger zone
                            let d1 = s_w - BOTTOM_RIGHT_F32.x + calc.smaller_straight
                                - 0.5 * l
                                - c.odo;
                            //exit danger zone
                            let d2 =
                                s_w - BOTTOM_RIGHT_F32.x + calc.bigger_straight + 0.5 * l
                                    - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.smaller_arc - SPACE_GAP - car.odo;
                            let dd = dist + calc.bigger_arc + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                        //
                        //from EAST RIGHT
                        if c.destination == Destination::RIGHT {
                            let calc = get_arc_len_with_straigh_path_three();
                            //enter danger zone
                            let d1 = s_w - BOTTOM_RIGHT_F32.x - 0.5 * l - c.odo;
                            //exit danger zone
                            let d2 = s_w - BOTTOM_RIGHT_F32.x + calc.arc_one - c.odo;
                            //out of danger zone
                            if d2 < 0.0 {
                                continue;
                            }
                            //time enter and exit danger zone
                            let t1 = d1 / c.speed;
                            let t2 = d2 / c.speed;
                            //main car
                            let d = dist + calc.arc_two - SPACE_GAP - car.odo;
                            let dd = dist + 0.5 * PI * BIG_RADIUS + 0.5 * l + SPACE_GAP - car.odo;
                            //out of danger zone
                            if dd < 0.0 {
                                continue;
                            }
                            //calculate speed range
                            let slow_speed = d / t2;
                            let fast_speed = dd / t1;
                            //add to speed intervals
                            let speed_interval = Interval::new(slow_speed, fast_speed);
                            speed_intervals.push(speed_interval);
                        }
                    }
                    //same origin (from SOUTH)
                    if c.origin == CARDINAL::SOUTH
                        && c.destination == Destination::RIGHT
                        && c.center.y > (dist + 3.5 * w + 7.0 * m)
                    {
                        //get car in front
                        if let Some(ref value) = car_in_front {
                            if value.center.y < c.center.y {
                                car_in_front = Some(c.clone());
                            }
                        } else {
                            if c.center.y < car.center.y {
                                car_in_front = Some(c.clone());
                            }
                        }
                    }
                }
            }
            //
            //adjust for car in front
            let mut max_speed: Option<f32> = None;
            if let Some(ref c_front) = car_in_front {
                //time for car in front to get to safe area
                let d = c_front.center.y - TOP_LEFT_F32.y;
                let t_front = d / c_front.speed;

                //speed that is needed to reach car in front
                let dd = car.center.y
                    - c_front.center.y
                    - SEPARATION_DISTANCE as f32
                    - CAR_LENGTH_F32;
                let v0 = dd / t_front;
                let v = v0 + c_front.speed;
                max_speed = Some(v);
            }

            //calculate speed based on time windows
            return get_speed_loop(dist, speed_intervals, max_speed);
        }

        return Some((TOP_LEFT_F32.x - CAR_LENGTH_F32 * 0.5) / MIN_TIME);
    }

    pub fn update_stat_max_min_time(statistics: &mut Statisics, elapsed: u128) {
        //
        //max time
        if let Some(max) = statistics.max_time {
            if elapsed > max {
                statistics.max_time = Some(elapsed);
            }
        } else {
            statistics.max_time = Some(elapsed);
        }
        //
        //min time
        if let Some(min) = statistics.min_time {
            if elapsed < min {
                statistics.min_time = Some(elapsed);
            }
        } else {
            statistics.min_time = Some(elapsed);
        }
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

        //Increase speed after junction
        let car_max_speed = (TOP_LEFT_F32.x + CAR_LENGTH_F32 * 0.5) / (MIN_TIME - 1.0);

        for car in &mut self.cars {
            if car.speed >= car_max_speed {
                continue;
            }
            if car.origin == CARDINAL::WEST && car.center.x > BOTTOM_RIGHT_F32.x {
                car.speed = car_max_speed;
                let elapsed = car.timer.elapsed().as_millis();
                Self::update_stat_max_min_time(&mut self.statistics, elapsed);
            }
            if car.origin == CARDINAL::EAST && car.center.x < TOP_LEFT_F32.x {
                car.speed = car_max_speed;
                let elapsed = car.timer.elapsed().as_millis();
                Self::update_stat_max_min_time(&mut self.statistics, elapsed);
            }
            if car.origin == CARDINAL::SOUTH && car.center.y < TOP_LEFT_F32.y {
                car.speed = car_max_speed;
                let elapsed = car.timer.elapsed().as_millis();
                Self::update_stat_max_min_time(&mut self.statistics, elapsed);
            }
            if car.origin == CARDINAL::NORTH && car.center.y > BOTTOM_RIGHT_F32.y {
                car.speed = car_max_speed;
                let elapsed = car.timer.elapsed().as_millis();
                Self::update_stat_max_min_time(&mut self.statistics, elapsed);
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

        //recalculate speed
        if self.counter % RE_CALCULATE_MOD == 0 {
            self.recalculate_first_cars();
        }

        


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

    fn recalculate_first_cars(&mut self) {
        let off_set = CAR_WIDTH_F32 * 3.5 + MARGIN_F32 * 7.0;
        let mut west_ahead: Option<Car> = None;
        let mut west_right: Option<Car> = None;
        let mut east_ahead: Option<Car> = None;
        let mut east_right: Option<Car> = None;
        let mut north_ahead: Option<Car> = None;
        let mut north_right: Option<Car> = None;
        let mut south_ahead: Option<Car> = None;
        let mut south_right: Option<Car> = None;
                
        let car_max_speed = (TOP_LEFT_F32.x + CAR_LENGTH_F32 * 0.5) / MIN_TIME;
        for c in self.cars.clone() {
            if c.speed >= car_max_speed {
                continue;
            }
            //
            if c.origin == CARDINAL::WEST {
                //
                if c.destination == Destination::AHEAD && c.center.x < BOTTOM_RIGHT_F32.x {
                    Self::update_front_car(&mut west_ahead, c.clone());
                }
                //
                if c.destination == Destination::RIGHT
                    && c.center.x < TOP_LEFT_F32.x + off_set
                {
                    Self::update_front_car(&mut west_right, c.clone());
                }
            }
            //
            if c.origin == CARDINAL::EAST {
                //
                if c.destination == Destination::AHEAD && c.center.x > TOP_LEFT_F32.x {
                    Self::update_front_car(&mut east_ahead, c.clone());
                }
                //
                if c.destination == Destination::RIGHT
                    && c.center.x > BOTTOM_RIGHT_F32.x - off_set
                {
                    Self::update_front_car(&mut east_right, c.clone());
                }
            }
            //
            if c.origin == CARDINAL::NORTH {
                //
                if c.destination == Destination::AHEAD && c.center.y < BOTTOM_RIGHT_F32.y {
                    Self::update_front_car(&mut north_ahead, c.clone());
                }
                //
                if c.destination == Destination::RIGHT
                    && c.center.y < TOP_LEFT_F32.y + off_set
                {
                    Self::update_front_car(&mut north_right, c.clone());
                }
            }
            //
            if c.origin == CARDINAL::SOUTH {
                //
                if c.destination == Destination::AHEAD && c.center.y > TOP_LEFT_F32.y {
                    Self::update_front_car(&mut south_ahead, c.clone());
                }
                //
                if c.destination == Destination::RIGHT
                    && c.center.y > BOTTOM_RIGHT_F32.y - off_set
                {
                    Self::update_front_car(&mut south_right, c.clone());
                }
            }
        }

        self.recalculate_speed_for_front_car(west_ahead);
        self.recalculate_speed_for_front_car(west_right);
        self.recalculate_speed_for_front_car(east_ahead);
        self.recalculate_speed_for_front_car(east_right);
        self.recalculate_speed_for_front_car(north_ahead);
        self.recalculate_speed_for_front_car(north_right);
        self.recalculate_speed_for_front_car(south_ahead);
        self.recalculate_speed_for_front_car(south_right);

        if self.cars.len() == 3 {
            self.cars.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
        }
    }

    pub fn update_front_car(target: &mut Option<Car>, c: Car) {
        if let Some(front_car) = target {
            if c.id < front_car.id {
                *target = Some(c);
            }
        } else {
            *target = Some(c);
        }
    }

    pub fn recalculate_speed_for_front_car(&mut self, car_option: Option<Car>) {
        if let Some(c) = car_option {
            self.cars.retain(|x| x.id != c.id);
            let mut c_clonned = c.clone();
            let speed_option = self.calculate_speed(&mut c_clonned);
            if let Some(speed) = speed_option {
                if speed > c.speed {
                    c_clonned.speed = speed;
                }
                self.cars.push(c_clonned);
            } else {
                self.cars.push(c_clonned);
            }
        }
    }

}

pub fn get_speed_loop(
    dist: f32,
    speed_intervals: Vec<Interval>,
    max_speed: Option<f32>,
) -> Option<f32> {
    let mut time = MIN_TIME;
    //
    //filter speed intervals
    let mut filtered_intervals = vec![];
    for interval in speed_intervals.clone() {
        if interval.start < 0.0 {
            continue;
        }
        if interval.end < 0.0 {
            let new_interval = Interval::new(interval.start, dist / MIN_TIME + 1.0);
            filtered_intervals.push(new_interval);
            continue;
        }

        filtered_intervals.push(interval);
    }

    loop {
        //calculate speed based on time;
        let v = dist / time;
        //
        //assume speed is ok
        let mut speed_ok = true;
        //
        //check speed intervals
        for interval in filtered_intervals.clone() {
            let slower_speed = interval.start;
            let faster_speed = interval.end;

            if v >= slower_speed && v <= faster_speed {
                speed_ok = false;
                break;
            }
        }
        //
        //check for max speed
        if let Some(m_sp) = max_speed {
            if v > m_sp {
                speed_ok = false;
            }
        }
        //
        //increase time if nesessery
        if speed_ok {
            return Some(v);
        }
        time += 1.0;
        if time >= 100000.0 {
            return None;
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
        let odo: f32 = -(CAR_LENGTH_F32 * 0.5);
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

#[derive(Debug, Clone)]
pub struct Interval {
    pub start: f32,
    pub end: f32,
}
impl Interval {
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }
}