use crate::{triangle::Triangle, vec3::Vec3, material::{MaterialEnum, Diffuse}};

#[derive(Clone, Debug)]
pub struct Hit {
    pub triangle: Triangle,
    pub at: Vec3,
    pub t: f64,
    pub material: MaterialEnum
}

impl Hit {
    pub fn new() -> Hit {
        return Hit {
            triangle: Triangle::new_empty(),
            at: Vec3::new(0.0, 0.0, -5000000000000.0),
            t: -1.0,
            material: MaterialEnum::Diffuse(Diffuse::new(Vec3::new(1.0, 1.0, 1.0)))
        };
    }
}