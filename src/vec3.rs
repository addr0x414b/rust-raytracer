use std::ops::{Mul, Add, Div, Sub, Neg, Index};

use rand::Rng;

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

    /// Return the length of a vector.
    pub fn length(self) -> f32 {
        return self.length_squared().sqrt();
    }

    /// Add the squared of every component in vector.
    pub fn length_squared(self) -> f32 {
        return (self.x()*self.x()) + (self.y()*self.y()) + (self.z()*self.z());
    }

    /// Debugging function to print the vector values to console.
    pub fn print(self) {
        println!("{}, {}, {}", self.x(), self.y(), self.z());
    }
}


/// Get the component of the vector based on the index. Iterate like an array.
impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.v[index];
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

/// Multiply two vectors and return result.
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        return Vec3::new(self.v[0] * v[0], self.v[1] * v[1], self.v[2] * v[2]);
    }
}

// Divide a Vec3 structs x,y,z values by a f32 float.
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        return self * (1.0 / t);
    }
}

/// Calculate the unit vector.
pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

/// Calculate the dot product of two vectors.
pub fn dot(a: Vec3, b: Vec3) -> f32 {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/// Calculate the cross product of two vectors.
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    return Vec3::new(
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    );
}

/// Generate a completely random vector
pub fn random_vec() -> Vec3 {
    let mut rng = rand::thread_rng();
    return Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
}

/// Generate a completely random vector with values in between min and max.
pub fn random_vec_range(min: f32, max: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    return Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max));
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.length_squared() <= 1.0 {
            return p;
        }
   }

   //let mut p = random_vec_range(-1.0, 1.0);
   //p = unit_vector(p);
   //return p;
}

pub type Color = Vec3;
pub type Point3 = Vec3;