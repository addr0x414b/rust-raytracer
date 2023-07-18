use crate::{triangle::Triangle, vec3::Vec3, material::{MaterialEnum, Diffuse}};

/// Contains information about the triangle the ray hit
#[derive(Clone, Debug)]
pub struct Hit {
    /// The hit triangle
    pub triangle: Triangle,
    /// The x,y,z coordinates of the position the ray hit in the triangle
    pub at: Vec3,
    /// Used to check if the ray hit a triangle
    pub t: f64,
    /// The triangle's material
    pub material: MaterialEnum
}

impl Hit {
    /// Create a new empty hit object
    /// # Default Values
    /// * 'triangle' - Empty triangle
    /// * 'at' - Hit position (0,0,-500000000)
    /// * 't' - -1.0
    /// * 'material' - White diffuse material
    pub fn new() -> Hit {
        return Hit {
            triangle: Triangle::new_empty(),
            at: Vec3::new(0.0, 0.0, -5000000000000.0),
            t: -1.0,
            material: MaterialEnum::Diffuse(Diffuse::new(Vec3::new(1.0, 1.0, 1.0)))
        };
    }
}