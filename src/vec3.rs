use std::ops::{Div, Sub, Add, Mul};
use rand::Rng;
use crate::hit::Hit;

/// Vec3 struct.
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {

    /// Create a new Vec3
    /// # Arguments
    /// 'x,y,z' - Positions
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z};
    }

    /// Return the length of the vector
    pub fn length(self) -> f64 {
        return self.length_squared().sqrt();
    }

    /// Calculate the length squared of the vector
    pub fn length_squared(self) -> f64 {
        return (self.x*self.x) + (self.y*self.y) + (self.z*self.z);
    }

    /// Check if the parameters of a Vec3 are very close to 0
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }
}

/// Allow the Vec3 to be divided by a double
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, n: f64) -> Self::Output {
        Vec3 {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

/// Allow the Vec3 to be subtracted by another Vec3
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Allow the Vec3 to be multiplied by a double
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, n: f64) -> Vec3 {
        return Vec3::new(self.x * n, self.y * n, self.z * n);
    }
}

/// Allow the Vec3 to be subtracted by another Vec3
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Calculate the cross product of two Vec3's
/// # Arguments
/// * 'a, b' - Two Vec3's 
/// # Returns
/// * The Vec3 result of the cross product
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    return Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z, 
        z: a.x * b.y - a.y * b.x,
    };
}

/// Calculate the dot product of two Vec3's
/// # Arguments
/// * 'a, b' - Two Vec3's 
/// # Returns
/// * The double result of the dot product
pub fn dot(a: Vec3, b: Vec3) -> f64 {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

/// Calculate the unit vector of a Vec3
/// # Arguments
/// * 'v' - The vector
/// # Returns
/// * The unit vector result
pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

/// Calculate the barycentric coordinates
/// # Arguments
/// * 'hit' - Hit struct which contains information of the triangle that was hit
/// # Returns
/// * The u,w,v barycentric results in the Vec3's x,y,z positions
pub fn barycentric(hit: Hit) -> Vec3 {
    let v0 = hit.triangle.points[1] - hit.triangle.points[0];
    let v1 = hit.triangle.points[2] - hit.triangle.points[0];
    let v2 = hit.at - hit.triangle.points[0];

    let d00 = dot(v0, v0);
    let d01 = dot(v0, v1);
    let d11 = dot(v1, v1);
    let d20 = dot(v2, v0);
    let d21 = dot(v2, v1);

    let denom = d00 * d11 - d01 * d01;

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    // Return the u,v,w in a Vec3 to grab 
    return Vec3::new(u, v, w);
}

/// Allow the Vec3 to be multiplied by another Vec3
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        return Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z);
    }
}

/// Calculate a random Vec3 given a range
/// # Arguments
/// * 'min, max' - Minimum and maximum values for the Vec3
/// # Returns
/// * A random Vec3
pub fn random_vec_range(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    return Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max));
}

/// Calculate a random vector in a unit sphere
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;

    }
}

/// Generate a random unit vector
pub fn random_unit_vector() -> Vec3 {
    return unit_vector(random_in_unit_sphere());
}

/// Reflect a Vec3 based on a Vec3 and a normal Vec3. Gives a perfect bounce
/// # Arguments
/// * 'v' - The incoming vector
/// * 'n' - The vector of the object we're hitting for the reflection
/// # Returns
/// * A new reflected vector
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - n * 2.0 * dot(v,n);
}