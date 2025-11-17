use crate::model::Point;

const SCALE: f32 = 1.0;
const SCREEN_WIDTH_DEFAULT: u32 = 690;
const SCREEN_HEIGHT_DEFAULT: u32 = 690;
pub static SCREEN_WIDTH: u32 = ((SCREEN_WIDTH_DEFAULT as f32) * SCALE) as u32;
pub static SCREEN_HEIGHT: u32 = ((SCREEN_HEIGHT_DEFAULT as f32) * SCALE) as u32;
pub static SCREEN_WIDTH_I32: i32 = SCREEN_WIDTH as i32;
pub static SCREEN_HEIGHT_I32: i32 = SCREEN_HEIGHT as i32;
pub static SCREEN_WIDTH_F32: f32 = SCREEN_WIDTH as f32;
pub static SCREEN_HEIGHT_F32: f32 = SCREEN_HEIGHT as f32;

const CAR_WIDTH_DEFAULT: u32 = 12;
const CAR_LENGTH_DEFAULT: u32 = 20;
pub static CAR_WIDTH: u32 = ((CAR_WIDTH_DEFAULT as f32) * SCALE) as u32;
pub static CAR_LENGTH: u32 = ((CAR_LENGTH_DEFAULT as f32) * SCALE) as u32;
pub static CAR_WIDTH_I32: i32 = CAR_WIDTH as i32;
pub static CAR_LENGTH_F32: f32 = CAR_LENGTH as f32;

pub static MARGIN_DEFAULT: u32 = 4;
pub static MARGIN: u32 = ((MARGIN_DEFAULT as f32) * SCALE) as u32;
pub static MARGIN_I32: i32 = MARGIN as i32;

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

pub const TOP_LEFT_URL: &str = "assets/images/top_left.png";
pub const FONT_URL: &str = "assets/fonts/arialnarrow.ttf";

pub static CAR_MAX_SPEED: f32 = 5.0;

pub static SEPARATION_DISTANCE: u32 = 10;
pub static SEPARATION_DISTANCE_F32: f32 = SEPARATION_DISTANCE as f32;

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
