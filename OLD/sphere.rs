use crate::hittable::{HitRecord, Hittable};
use crate::vec3::{Point3, Vec3, dot};
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Sphere {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = dot(oc, r.direction());
        let c: f32 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f32 = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd: f32 = discriminant.sqrt();

        let root: f32 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        return true;
    }
}