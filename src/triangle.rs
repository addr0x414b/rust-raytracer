use crate::{vec3::{Vec3, cross, dot}, ray::Ray, hit::Hit};

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub points: [Vec3; 3],
    pub normal: Vec3,
    pub smooth: bool,
    pub normals: [Vec3; 3],
}

impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3, n: Vec3) -> Triangle {
        return Triangle {
            points: [p1, p2, p3],
            normal: n,
            smooth: false,
            normals: [Vec3::new(0.0, 0.0, 0.0); 3],
        };
    }

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
    pub fn hit(&self, r: Ray) -> Hit {
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