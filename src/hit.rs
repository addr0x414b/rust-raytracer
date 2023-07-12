use crate::{triangle::Triangle, vec3::{Point3, Color}, mesh::MaterialEnum, material::Diffuse};

/// Contains information about the triangle we hit
#[derive(Clone)]
pub struct Hit {
    /// Store the triangle we hit
    pub triangle: Triangle,
    /// The x,y,z coordinates of the position we hit inside of the triangle
    pub at: Point3,
    /// Used to calculate 'at'. Also used to check if we even hit a triangle
    pub t: f32,
    /// The hit triangle's material
    pub material: MaterialEnum,
}

impl Hit {
    /// Create a new empty hit 'object'
    /// # Default Values
    /// * 'triangle' - An empty triangle
    /// * 'at' - Vec3 with (x,y,z) of (0.0, 0.0, -50000000.0)
    /// * 't' - -1.0
    pub fn new() -> Hit {
        Hit {
            triangle: Triangle::new_empty(),
            // We want to draw what's closest to the camera. Z value closer to 0 = closer to us
            // Therefore set default z super small so that it's really far away
            at: Point3::new(0.0, 0.0, -5000000.0),
            t: -1.0,
            material: MaterialEnum::Diffuse(Diffuse::new(Color::new(0.0,0.0,0.0)))
        }
    }
}