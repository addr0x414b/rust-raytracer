use crate::vec3::{Vec3, Point3};

/// A struct that symbolizes a ray
#[derive(Copy, Clone)]
pub struct Ray {
    /// The ray origin
    pub origin: Point3,
    /// The ray direction
    pub direction: Vec3
}

impl Ray {
    /// Create new Ray
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    /// Calculate the 3d coordinates of a position on the ray based on 't'
    pub fn at(&self, t: f32) -> Point3 {
        return self.origin + (self.direction*t);
    }
}