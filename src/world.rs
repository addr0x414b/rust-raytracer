use crate::{mesh::Mesh, ray::Ray, hit::Hit, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct World {
    pub meshes: Vec<Mesh>
}

impl World {
    pub fn new() -> World {
        return World { meshes: Vec::new() };
    }

    pub fn add(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}

impl World {
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