use crate::{mesh::Mesh, ray::Ray, hit::Hit};

/// World struct
#[derive(Clone, Debug)]
pub struct World {
    /// All the meshes in the world
    pub meshes: Vec<Mesh>
}

impl World {
    /// Create a new empty world
    pub fn new() -> World {
        return World { meshes: Vec::new() };
    }

    /// Add a mesh to the world
    pub fn add(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}

impl World {

    /// Check if any object in the world is hit by a ray
    /// # Arguments
    /// * 'r' - The incoming ray
    /// # Returns
    /// * A hit struct containing the closest hit triangle and its properties
    pub fn hit(&self, r: Ray) -> Hit {
        let mut closest_hit = Hit::new();
        for mesh in self.meshes.iter() {
            let hit = mesh.hit(r);
            if hit.t > 0.0 {
                if hit.at.z > closest_hit.at.z {
                    closest_hit = hit;
                }
            }
        }
        return closest_hit;
    }
}