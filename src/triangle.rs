use crate::{vec3::{Vec3, cross, dot}, ray::Ray, hit::Hit};

/// Triangle struct
#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    /// The 3 points of a triangle
    pub points: [Vec3; 3],
    /// Flat shaded triangles single normal vector
    pub normal: Vec3,
    /// Whether or not the triangle is smoothly shaded
    pub smooth: bool,
    /// If smooth, contains 3 normals for the 3 points
    pub normals: [Vec3; 3],
}

impl Triangle {

    /// Create a new triangle, default flat shaded
    /// # Arguments
    /// * 'p1, p2, p3' - The three points of the triangle 
    /// * 'n' - Triangle normal vector
    /// # Returns
    /// * Triangle with given points and normal, smooth is default off and empty normals per vertex
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3, n: Vec3) -> Triangle {
        return Triangle {
            points: [p1, p2, p3],
            normal: n,
            smooth: false,
            normals: [Vec3::new(0.0, 0.0, 0.0); 3],
        };
    }

    /// Create an empty triangle
    /// # Returns
    /// * Triangle with (0,0,0) for all points and normals
    pub fn new_empty() -> Triangle {
        return Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        );
    }
}

impl Triangle {

    /// Check if the triangle has been hit by the ray
    /// # Arguments
    /// * 'r' - The incoming ray
    /// # Returns
    /// * Hit struct containing all the information of the triangle
    /// # Credit
    /// * Using Möller–Trumbore intersection algorithm
    /// * The code was provided by Wikipedia in C++, translated by me
    /// * <https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm>
    pub fn hit(&self, r: Ray) -> Hit {

        // Create an empty hit object, this will get populated the ray hits the triangle
        let mut hit = Hit::new();
        let edge1 = self.points[1] - self.points[0];
        let edge2 = self.points[2] - self.points[0];
        let h = cross(r.direction, edge2);
        let a = dot(edge1, h);
        const EPSILON: f64 = 0.0000001;
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

        // If this is true, this means the ray hit the triangle
        if t > EPSILON {
            let mut trig = Triangle::new_empty();
            trig.points = self.points;
            trig.normal = self.normal;

            if self.smooth {
                trig.smooth = true;
                trig.normals = self.normals;
            }

            hit.triangle = trig;
            hit.t = t;
            hit.at = r.at(t);
            return hit;
        }
        else {
            return hit;
        }

    }
}