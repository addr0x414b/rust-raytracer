use std::ops::Index;

use crate::{vec3::{Point3, Vec3, cross, dot}, ray::Ray, hit::Hit};

/// A struct that contains information for a triangle
#[derive(Copy, Clone)]
pub struct Triangle {
    /// The 3 points of a triangle
    pub points: [Point3; 3],
    /// The triangle's normal vector. Only use if 'smooth' is set to false
    pub normal: Vec3,
    /// True if the triangle is smoothly shaded
    pub smooth: bool,
    /// Correctly populated if our triangle is smoothly shaded. In other words,
    /// only use if the 'smooth' value is set to true
    pub normals: [Point3; 3],
}

impl Triangle {
    /// Create a new FLAT SHADED triangle given 3 points and a normal
    pub fn new(a: Point3, b: Point3, c: Point3, n: Vec3) -> Triangle {
        Triangle { points: [a,b,c], normal: n, smooth: false, normals: [a,b,c] }
    }

    /// Create an empty triangle.
    pub fn new_empty() -> Triangle {
        return Triangle::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        );
    }

    /// Check if the triangle has been hit by the ray
    /// Return the hit struct which contains values about the triangle
    /// # Credit
    /// * Using Möller–Trumbore intersection algorithm
    /// * The code was provided by Wikipedia in C++, translated by me
    /// * <https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm>
    pub fn hit(self, r: Ray) -> Hit {

        // Create an empty hit object. If the ray hits a triangle, this will get populated
        let mut hit: Hit = Hit::new();
        let edge1 = self.points[1] - self.points[0];
        let edge2 = self.points[2] - self.points[0];
        let h = cross(r.direction, edge2);
        let a = dot(edge1, h);
        const EPSILON: f32 = 0.0000001;
        if a > -EPSILON && a < EPSILON {
            return hit;
        }

        let f = 1.0 / a;
        let s = r.origin - self.points[0];
        let u = f * dot(s, h);
        if u < 0.0 || u > 1.0 {
            return hit;
        }

        let q = cross(s, edge1);
        let v = f * dot(r.direction, q);
        if v < 0.0 || u + v > 1.0 {
            return hit;
        }

        let t = f * dot(edge2, q);
        // If t > EPSILON, that means our ray has hit the triangle
        if t > EPSILON {
            // Create a new empty triangle. This will be used for our 'hit' struct
            let mut trig: Triangle = Triangle::new_empty();
            trig.points = self.points;
            trig.normal = self.normal;

            if self.smooth {
                trig.smooth = true;
                trig.normals = [self.normals[0], self.normals[1], self.normals[2]];
            }
            
            // Populate our hit struct with the triangle, t value, and the 'at' position
            // where the ray actually intersects the triangle
            hit.triangle = trig;
            hit.t = t;
            let at: Point3 = r.at(t);
            hit.at = at;
            return hit; // Return the hit struct
        }
        else {
            return hit;
        }
    }
}

/// Get the point of a triangle based on the index. Iterate like an array
impl Index<usize> for Triangle {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.points[index];
    }
}