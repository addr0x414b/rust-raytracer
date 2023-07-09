use crate::{triangle::Triangle, ray::Ray, hit::Hit};

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

    pub fn hit(&self, r: Ray) -> Hit {

        let mut closest_hit: Hit = Hit::new();
        for trig in self.triangles.iter() {
            let hit: Hit = trig.hit(r);
            if hit.t > 0.0 {
                if hit.at.z() > closest_hit.at.z() {
                    closest_hit = hit;
                }
            }
        }
        return closest_hit;
    }
}