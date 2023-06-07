use std::ops::{Neg, Index, AddAssign, MulAssign};

// Initialize Vec3 struct. Contains an array of 3 floats: x, y, z
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    v: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { v: [x, y, z]}
    }

    // Functions to receive x, y, and z values
    pub fn x(&self) -> f32 {
        return self.v[0];
    }
    pub fn y(&self) -> f32 {
        return self.v[1];
    }
    pub fn z(&self) -> f32 {
        return self.v[2];
    }
}

// Negate a vec3, should flip all values
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3::new(-self.v[0], -self.v[1], -self.v[2]);
    }
}

// Directly get the point value based on index
impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.v[i];
    }
}

// Add equals to self
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.v[0] += v.v[0];
        self.v[1] += v.v[1];
        self.v[2] += v.v[2];
    }
}

// Multiply single float to self
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, n: f32) {
        self.v[0] *= n;
        self.v[1] *= n;
        self.v[2] *= n;
    }
}