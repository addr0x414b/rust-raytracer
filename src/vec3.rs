
/// A struct that stores an array of three f32's. This struct is used for 
/// vector 3's, points in 3d space, as well as RGB color values.
pub struct Vec3 {
    v: [f32; 3],
}

impl Vec3 {
    /// Generate new Vec3 struct instance.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {v: [x, y, z]}
    }

    /// Get the first (x) value in our vector.
    pub fn x(&self) -> f32 {
        return self.v[0];
    }

    /// Get the second (y) value in our vector.
    pub fn y(&self) -> f32 {
        return self.v[1];
    }

    /// Get the third (z) value in our vector.
    pub fn z(&self) -> f32 {
        return self.v[2];
    }
}

pub type Color = Vec3;