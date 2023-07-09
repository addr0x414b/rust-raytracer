use std::ops::Index;

use crate::{vec3::{Point3, Vec3, cross, dot}, ray::Ray, hit::Hit};

/// A struct that contains information for a triangle. Has an array of 
/// 3 poins, and a normal vector.
#[derive(Copy, Clone)]
pub struct Triangle {
    points: [Point3; 3],
    normal: Vec3,
}

impl Triangle {
    /// Create a new triangle.
    pub fn new(a: Point3, b: Point3, c: Point3, n: Vec3) -> Triangle {
        Triangle { points: [a,b,c], normal: n }
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

    /// Get the triangles normal vector.
    pub fn normal(self) -> Vec3 {
        return self.normal;
    }

    /// Check if the triangle has been hit by the ray.
    /// Return the hit struct which contains values about the triangle.
    pub fn hit(self, r: Ray) -> Hit {
        let mut hit: Hit = Hit::new();

        let edge1 = self.points[1] - self.points[0];
        let edge2 = self.points[2] - self.points[0];
        let h = cross(r.direction(), edge2);
        let a = dot(edge1, h);
        const EPSILON: f32 = 0.0000001;
        if a > -EPSILON && a < EPSILON {
            return hit;
        }

        let f = 1.0 / a;
        let s = r.origin() - self.points[0];
        let u = f * dot(s, h);
        if u < 0.0 || u > 1.0 {
            return hit;
        }

        let q = cross(s, edge1);
        let v = f * dot(r.direction(), q);
        if v < 0.0 || u + v > 1.0 {
            return hit;
        }

        let t = f * dot(edge2, q);
        if t > EPSILON {
            let mut trig: Triangle = Triangle::new_empty();
            trig.points = self.points;
            trig.normal = self.normal();

            hit.triangle = trig;
            hit.t = t;

            let at: Point3 = r.at(t);

            hit.at = at;
            return hit;
        }
        else {
            return hit;
        }
    }
}

/// Get the point of a triangle based on the index. Iterate like an array.
impl Index<usize> for Triangle {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.points[index];
    }
}