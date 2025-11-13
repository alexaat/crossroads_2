const SCALE: f32 = 1.0;
const SCREEN_WIDTH_DEFAULT: u32 = 690;
const SCREEN_HEIGHT_DEFAULT: u32 = 690;
pub static SCREEN_WIDTH: u32 = ((SCREEN_WIDTH_DEFAULT as f32) * SCALE) as u32;
pub static SCREEN_HEIGHT: u32 = ((SCREEN_HEIGHT_DEFAULT as f32) * SCALE) as u32;
pub static SCREEN_WIDTH_I32: i32 = SCREEN_WIDTH as i32; 
pub static SCREEN_HEIGHT_I32: i32 = SCREEN_HEIGHT as i32; 

const CAR_WIDTH_DEFAULT: u32 = 12;
const CAR_LENGTH_DEFAULT: u32 = 20;
pub static CAR_WIDTH: u32 = ((CAR_WIDTH_DEFAULT as f32) * SCALE) as u32;
pub static CAR_LENGTH: u32 = ((CAR_LENGTH_DEFAULT as f32) * SCALE) as u32;
pub static CAR_WIDTH_I32: i32 = CAR_WIDTH as i32;

pub static MARGIN_DEFAULT: u32 = 4;
pub static MARGIN: u32 = ((MARGIN_DEFAULT as f32) * SCALE) as u32;
pub static MARGIN_I32: i32 = MARGIN as i32;

pub const LINE_LENGHT: i32 = 40;
pub const LINE_GAP: i32 = 25;
