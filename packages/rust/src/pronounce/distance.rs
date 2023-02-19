const VOWEL_WEIGHT: f32 = 2.0;
const CONSONANT_WEIGHT: f32 = 0.25;

pub fn vowel_distance(x_1: &f32, x_2: &f32, y_1: &f32, y_2: &f32) -> f32 {
    let x_diff = (x_1 - x_2).abs();
    let y_diff = (y_1 - y_2).abs();
    let imperfect_punish = if x_diff != 0.0 && y_diff != 0.0 {
        0.1
    } else {
        0.0
    };
    ((x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt() + imperfect_punish) * VOWEL_WEIGHT
}

pub fn consonant_distance(x_1: &f32, x_2: &f32, y_1: &f32, y_2: &f32) -> f32 {
    let x_diff = (x_1 - x_2).abs();
    let y_diff = (y_1 - y_2).abs();
    let row_punish = if y_diff == 0.0 { 0.0 } else { 0.5 };
    let col_punish = if x_diff == 0.0 { 0.0 } else { 0.1 };
    ((x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt() + row_punish + col_punish) / 1.6
        * CONSONANT_WEIGHT
}

pub fn silent_consonant_distance(y: &f32) -> f32 {
    // the close to 0.5 the more silent it is, because that's where fricatives are
    // the more silent it is, the less the distance
    let consonant_strength = 2.0 * (y - 0.5).abs();
    consonant_strength * CONSONANT_WEIGHT + 0.15
}
