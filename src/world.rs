use crate::mesh::Mesh;

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
}