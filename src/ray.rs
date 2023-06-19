use crate::vec3::{Vec3, Point3};

/// A struct that symbolizes a ray. It contains a 3D point as the origin,
/// and a direction which is a vector.
pub struct Ray{
    origin: Point3,
    direction: Vec3
}

impl Ray {
    /// Generate new Ray.
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    /// Get the ray origin.
    pub fn origin(&self) -> Point3 {
        return self.origin;
    }

    /// Get the ray direction.
    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }

    pub fn at(&self, t: f32) -> Point3 {
        return self.origin + (self.direction*t);
    }
}