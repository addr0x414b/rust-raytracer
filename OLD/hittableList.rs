use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub  fn new() -> Self {
        HittableList { objects: Vec::new() }
    }
    
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
        fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
            let mut temp_rec: HitRecord = HitRecord::default();
            let mut hit_anything: bool = false;
            let mut closest_so_far: f32 = t_max;

            for object in &self.objects {
                if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    *rec = temp_rec.clone();
                }
            }
            return hit_anything;
        }
}