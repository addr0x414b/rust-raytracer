pub use std::f32::{INFINITY, consts::PI};
use rand::Rng;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen::<f32>();
}

pub fn random_double_rangle(min: f32, max: f32) -> f32 {
    return min + (max - min) * random_double();
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return max;

}