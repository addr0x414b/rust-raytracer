use crate::{triangle::Triangle, ray::Ray};

/// A mesh that is rendered in the world.
/// Contains a vector of triangles to be drawn.
#[derive(Clone)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    /// Create a default empty mesh. Can add triangles to it later.
    pub fn new() -> Mesh {
        Mesh { triangles: Vec::new() }
    }

    /// Create a mesh with an already established Vec of triangles.
    pub fn new_mesh(trigs: Vec<Triangle>) -> Mesh {
        Mesh { triangles: trigs }
    }

    pub fn hit(&self, r: Ray, t: &mut Triangle) -> f32 {

        for trig in self.triangles.iter() {
            let t = trig.hit(r, t);
            if t > 0.0 {
                return t;
            }
        }
        return -1.0;
    }
}