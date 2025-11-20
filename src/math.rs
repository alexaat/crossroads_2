use crate::preferences::*;

pub fn get_arc_len_with_straigh_path_one() -> ArcCalc {
    let mut result = vec![];
    let w = CAR_WIDTH_F32;
    let m = MARGIN_F32;
    let l = CAR_LENGTH_F32;

    let x1 = 0.5 * l;
    let y1 = w * 3.0 + m * 6.0;
    let r1 = (x1.powi(2) + y1.powi(2)).sqrt();
    //
    let sin_a0 = x1 / r1;
    let a0 = sin_a0.asin();
    //
    let cos_a = (w * 2.0 + m * 4.0) / r1;
    let a = cos_a.acos();
    //
    let delta_a = a - a0;
    result.push(delta_a * BIG_RADIUS);
    //
    let y2 = w * 4.0 + m * 8.0;
    let r2 = (x1.powi(2) + y2.powi(2)).sqrt();
    //
    let sin_a0 = x1 / r2;
    let a0 = sin_a0.asin();
    //
    let cos_a = (w + m * 2.0) / r2;
    let a = cos_a.acos();
    //
    let delta_a = a - a0;
    result.push(delta_a * BIG_RADIUS);

    //calculate other car
    let d_bigger = (r2.powi(2) - (w + 2.0 * m).powi(2)).sqrt();
    let d_smaller = (r1.powi(2) - (2.0 * w + 4.0 * m).powi(2)).sqrt();
    result.push(d_bigger);
    result.push(d_smaller);

    let arc_calc = ArcCalc {
        smaller_arc: result[0],
        bigger_arc: result[1],
        bigger_straight: result[2],
        smaller_straight: result[3],
    };
    arc_calc
}

pub fn get_arc_len_with_straigh_path_two() -> ArcCalc {
    let w = CAR_WIDTH_F32;
    let m = MARGIN_F32;
    let l = CAR_LENGTH_F32;

    let x1 = 0.5 * l;
    let y1 = w * 3.0 + m * 6.0;
    let r1 = (x1.powi(2) + y1.powi(2)).sqrt();
    //
    let y2 = w * 4.0 + m * 8.0;
    let r2 = (x1.powi(2) + y2.powi(2)).sqrt();
    //
    let sin_a0 = x1 / r2;
    let a0 = sin_a0.asin();
    //
    let sin_a = (w + 2.0 * m) / r2;
    let a = sin_a.asin();
    let delta_a = a - a0;
    let smaller_arc = delta_a * BIG_RADIUS;
    //
    let sin_a0 = x1 / r1;
    let a0 = sin_a0.asin();
    let sin_a = (2.0 * w + 4.0 * m) / r1;
    let a = sin_a.asin();
    let delta_a = a + a0;
    let bigger_arc = delta_a * BIG_RADIUS;

    //calculate other car
    let bigger_straight = (r2.powi(2) - (w + 2.0 * m).powi(2)).sqrt();
    let smaller_straight = (r1.powi(2) - (2.0 * w + 4.0 * m).powi(2)).sqrt();
    let arc_calc = ArcCalc {
        smaller_arc,
        bigger_arc,
        bigger_straight,
        smaller_straight,
    };
    arc_calc
}

pub fn get_arc_len_with_straigh_path_three() -> ArcCalcTwo {
    let w = CAR_WIDTH_F32;
    let m = MARGIN_F32;
    let l = CAR_LENGTH_F32;
    let h_total = w * 6.0 + m * 12.0;

    let x1 = 0.5 * l;
    let y1 = w * 3.0 + m * 6.0;
    let r1 = (x1.powi(2) + y1.powi(2)).sqrt();
    //
    let y2 = w * 4.0 + m * 8.0;
    let r2 = (x1.powi(2) + y2.powi(2)).sqrt();
    //
    let sin_a0 = x1 / r1;
    let a0 = sin_a0.asin();
    //
    //
    let h2 = (h_total.powi(2) + r1.powi(2) - r2.powi(2)) / (2.0 * h_total);
    //
    let cos_a = h2 / r1;
    let a = cos_a.acos();
    //
    let delta_a = a + a0;
    let arc_one = delta_a * BIG_RADIUS;
    //other car
    let sin_a = h2 / r1;
    let a = sin_a.asin();
    //
    let delta_a = a - a0;
    let arc_two = delta_a * BIG_RADIUS;
    let arc_calc_two = ArcCalcTwo { arc_one, arc_two };
    arc_calc_two
}

pub struct ArcCalc {
    pub bigger_arc: f32,
    pub smaller_arc: f32,
    pub bigger_straight: f32,
    pub smaller_straight: f32,
}
pub struct ArcCalcTwo {
    pub arc_one: f32,
    pub arc_two: f32,
}