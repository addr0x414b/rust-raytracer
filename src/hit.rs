use crate::{triangle::Triangle, vec3::Point3};

/// Contains information about the triangle we hit.
#[derive(Copy, Clone)]
pub struct Hit {
    pub triangle: Triangle,
    pub at: Point3,
    pub t: f32
}

impl Hit {
    pub fn new() -> Hit {
        Hit {
            triangle: Triangle::new_empty(),
            at: Point3::new(0.0, 0.0, -5000000.0),
            t: -1.0
        }
    }
}