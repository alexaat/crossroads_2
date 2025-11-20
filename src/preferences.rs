use crate::model::Point;
use crate::model::PointF;

const SCALE: f32 = 1.0;
const SCREEN_WIDTH_DEFAULT: u32 = 690;
const SCREEN_HEIGHT_DEFAULT: u32 = 690;
pub static SCREEN_WIDTH: u32 = ((SCREEN_WIDTH_DEFAULT as f32) * SCALE) as u32;
pub static SCREEN_HEIGHT: u32 = ((SCREEN_HEIGHT_DEFAULT as f32) * SCALE) as u32;
pub static SCREEN_WIDTH_I32: i32 = SCREEN_WIDTH as i32;
pub static SCREEN_HEIGHT_I32: i32 = SCREEN_HEIGHT as i32;
pub static SCREEN_WIDTH_F32: f32 = SCREEN_WIDTH as f32;
pub static SCREEN_HEIGHT_F32: f32 = SCREEN_HEIGHT as f32;

pub const CAR_WIDTH_DEFAULT: u32 = 12;
pub const CAR_LENGTH_DEFAULT: u32 = 20;
pub static CAR_WIDTH: u32 = ((CAR_WIDTH_DEFAULT as f32) * SCALE) as u32;
pub static CAR_LENGTH: u32 = ((CAR_LENGTH_DEFAULT as f32) * SCALE) as u32;
pub static CAR_LENGTH_I32: i32 = CAR_LENGTH as i32;
pub static CAR_WIDTH_I32: i32 = CAR_WIDTH as i32;
pub static CAR_LENGTH_F32: f32 = CAR_LENGTH as f32;
pub static CAR_WIDTH_F32: f32 = CAR_WIDTH as f32;

pub static MARGIN_DEFAULT: u32 = 4;
pub static MARGIN: u32 = ((MARGIN_DEFAULT as f32) * SCALE) as u32;
pub static MARGIN_I32: i32 = MARGIN as i32;
pub static MARGIN_F32: f32 = MARGIN as f32;

pub const LINE_LENGHT: i32 = 40;
pub const LINE_GAP: i32 = 25;

pub static TOP_LEFT: Point = Point {
    x: (SCREEN_WIDTH_I32 - 6 * (CAR_WIDTH_I32 + 2 * MARGIN_I32)) / 2,
    y: (SCREEN_HEIGHT_I32 - 6 * (CAR_WIDTH_I32 + 2 * MARGIN_I32)) / 2,
};

pub static BOTTOM_RIGHT: Point = Point {
    x: (SCREEN_WIDTH_I32 + 6 * (CAR_WIDTH_I32 + 2 * MARGIN_I32)) / 2,
    y: (SCREEN_HEIGHT_I32 + 6 * (CAR_WIDTH_I32 + 2 * MARGIN_I32)) / 2,
};

pub static TOP_LEFT_F32: PointF = PointF {
    x: TOP_LEFT.x as f32,
    y: TOP_LEFT.y as f32,
};

pub static BOTTOM_RIGHT_F32: PointF = PointF {
    x: BOTTOM_RIGHT.x as f32,
    y: BOTTOM_RIGHT.y as f32,
};

pub const TOP_LEFT_URL: &str = "assets/images/top_left.png";
pub const FONT_URL: &str = "assets/fonts/arialnarrow.ttf";

pub static CAR_MAX_SPEED: f32 = 5.0;

pub static SEPARATION_DISTANCE: u32 = 10;
pub static SEPARATION_DISTANCE_F32: f32 = SEPARATION_DISTANCE as f32;

pub static SMALL_RADIUS: u32 = MARGIN + CAR_WIDTH / 2;
pub static BIG_RADIUS: f32 = CAR_WIDTH as f32 * 3.5 + MARGIN as f32 * 7.0;
pub static SMALL_RADIUS_F32: f32 = SMALL_RADIUS as f32;
pub static BIG_RADIUS_F32: f32 = BIG_RADIUS as f32;

pub static CAR_URLS: [&str; 3] = [
    "assets/images/blue_car_20.png",
    "assets/images/red_car_20.png",
    "assets/images/green_car_20.png",
];

pub static MIN_TIME: f32 = 50.0;
pub static SPACE_GAP: f32 = 20.0;

pub static RE_CALCULATE_MOD: u64 = 20;

/*

    ____________________                         _____________________
   |         285        | 20| 20| 20| 20| 20| 20|
   |                    |   |   |   |   |   |   |
   |                    |   |   |   |   |   |   |
   |285                 |   |   |   |   |   |   |
   |                    |   |   |   |   |   |   |
   |                    |   |   |   |   |   |   |
   |                    |   |   |   |   |   |   |
   |____________________                         _____________________
        20          ->
    ____________________
        20          ->
    ____________________
        20          ->
    ____________________
        20          <-
    ____________________
        20          <-
    ____________________
        20          <-
    ____________________
   |
   |
   |
   |
   |285
   |
   |
   |
   |___________________________




*/
