use crate::vec3::Vec3;

/// Ray struct
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    /// Where the ray begins
    pub origin: Vec3,
    /// The direction the ray is pointing
    pub direction: Vec3
}

impl Ray {

    /// Create a new ray
    /// # Arguments
    /// * 'origin' - Ray starting position
    /// * 'direction' - Ray direction
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        return Ray { origin, direction };
    }

    /// Calculate the at position on a ray given t
    /// # Arguments
    /// * 't' - Constant on a ray to calculate
    /// # Returns
    /// * Vec3 position on the ray given a constant t
    pub fn at(&self, t: f64) -> Vec3 {
        return self.origin + (self.direction * t);
    }
}