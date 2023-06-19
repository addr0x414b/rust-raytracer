use std::ops::{Mul, Add, Div, Sub, Neg};

/// A struct that stores an array of three f32's. This struct is used for 
/// vector 3's, points in 3d space, as well as RGB color values.
#[derive(Copy, Clone)]
pub struct Vec3 {
    v: [f32; 3],
}

impl Vec3 {
    /// Generate new Vec3 struct instance.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {v: [x, y, z]}
    }

    /// Get the first (x) value in our vector.
    pub fn x(self) -> f32 {
        return self.v[0];
    }

    /// Get the second (y) value in our vector.
    pub fn y(self) -> f32 {
        return self.v[1];
    }

    /// Get the third (z) value in our vector.
    pub fn z(self) -> f32 {
        return self.v[2];
    }

    /// Debugging function to print the vector values to console.
    pub fn print(self) {
        println!("{}, {}, {}", self.x(), self.y(), self.z());
    }
}

/// Flip all the values sign in a vector.
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3::new(-self.x(), -self.y(), -self.z());
    }
}

/// Add two Vec3 structs x,y,z values and return the sum.
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        return Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z());
    }
}

/// Subtract two Vec3 structs x,y,z values.
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        return self + (-v);
    }
}

/// Multiply a Vec3 structs x,y,z values by a f32 float.
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        return Vec3::new(self.x()*t, self.y()*t, self.z()*t);
    }
}

// Divide a Vec3 structs x,y,z values by a f32 float.
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        return self * (1.0 / t);
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;