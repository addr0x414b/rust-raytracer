use std::ops::{Neg, Index, AddAssign, Add, Sub, MulAssign, Mul, DivAssign, Div};

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

    // Print x, y, z
    pub fn print(&self) {
        println!("{}, {}, {}", self.v[0], self.v[1], self.v[2]);
    }

    // Functions to get length and squared length
    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }
    pub fn length_squared(&self) -> f32 {
        return self.v[0]*self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2];
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

// Add two vectors and return result
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        return Vec3::new(self.v[0] + v[0], self.v[1] + v[1], self.v[2] + v[2]);
    }
}

// Subtract two vectors and return result
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        return Vec3::new(self.v[0] - v[0], self.v[1] - v[1], self.v[2] - v[2]);
    }
}

// Multiply equals single float to self
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, n: f32) {
        self.v[0] *= n;
        self.v[1] *= n;
        self.v[2] *= n;
    }
}

// Multiply two vectors and return result
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        return Vec3::new(self.v[0] * v[0], self.v[1] * v[1], self.v[2] * v[2]);
    }
}

// Multiply vector by constant and return result
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, c: f32) -> Vec3 {
        return Vec3::new(self.v[0] * c, self.v[1] * c, self.v[2] * c);
    }
}

// Divide equals float to self
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, n: f32) {
        *self *= 1.0 / n;
    }
}

// Divide vector by constant and return result
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, c: f32) -> Vec3 {
        return Vec3::new(self.v[0] * (1.0 / c), self.v[1] * (1.0 / c), self.v[2] * (1.0 / c));
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    return a.v[0] * b.v[0] + a.v[1] * b.v[1] + a.v[2] * b.v[2];
}

pub fn cross(a: Vec3, b:Vec3) -> Vec3 {
    return Vec3::new(a.v[1] * b.v[2] - a.v[2] * b.v[1], a.v[2] * b.v[0] - a.v[0] * b.v[2], a.v[0] * b.v[1] - a.v[1] * b.v[0]);
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

pub type Point3 = Vec3;
pub type Color = Vec3;