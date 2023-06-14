pub use std::f32::{INFINITY, consts::PI};

fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}