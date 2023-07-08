use crate::{mesh::Mesh, ray::Ray, triangle::Triangle};

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
    pub fn hit(&self, r: Ray, trig: &mut Triangle) -> f32 {

        for mesh in self.meshes.iter() {
            let t = mesh.hit(r, trig);
            if t > 0.0 {
                return t;
            }
        }
        return -1.0;
    }
}