use crate::{mesh::Mesh, ray::Ray, triangle::Triangle, hit::Hit};

/// The world. Contains all the meshes in the scene.
#[derive(Clone)]
pub struct World {
    meshes: Vec<Mesh>,
}

impl World {

    /// Create a new empty world.
    pub fn new() -> World {
        World { meshes: Vec::new() }
    }

    /// Add a mesh to the world.
    pub fn add(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    /// Iterate over all meshes and check if the ray has hit.
    pub fn hit(&self, r: Ray) -> Hit {

        let mut closest_hit: Hit = Hit::new();
        for mesh in self.meshes.iter() {
            let hit: Hit = mesh.hit(r);
            if hit.t > 0.0 {
                if hit.at.z() > closest_hit.at.z() {
                    closest_hit = hit;
                }
            }
        }
        return closest_hit;
    }
}