use crate::{triangle::Triangle, ray::Ray, hit::Hit, vec3::{Vec3, Point3}};

/// A mesh that is rendered in the world.
/// Contains a vector of triangles to be drawn.
#[derive(Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
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

    pub fn new_plane() -> Mesh {
        // Bot triangles
        let t1: Triangle = Triangle::new(
            Point3::new(-1.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0)
        );
        let t2: Triangle = Triangle::new(
            Point3::new(-1.0, 0.0, 1.0),
            Point3::new(-1.0, 0.0, -1.0),
            Point3::new(1.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0)
        );

        let plane: Mesh = Mesh::new_mesh(vec![
            t1,t2
            ]);
        return plane;
    }

    /// Create a default cube
    pub fn new_cube() -> Mesh {
        // Front triangles
        let t1: Triangle = Triangle::new(
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        let t2: Triangle = Triangle::new(
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(-1.0, 1.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0)
        );

        // Right triangles
        let t3: Triangle = Triangle::new(
            Point3::new(1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(1.0, 1.0, -1.0),
            Vec3::new(1.0, 0.0, 0.0)
        );
        let t4: Triangle = Triangle::new(
            Point3::new(1.0, -1.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(1.0, 1.0, -1.0),
            Vec3::new(1.0, 0.0, 0.0)
        );

        // Left triangles
        let t5: Triangle = Triangle::new(
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(-1.0, 1.0, 1.0),
            Vec3::new(-1.0, 0.0, 0.0)
        );
        let t6: Triangle = Triangle::new(
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(-1.0, 1.0, -1.0),
            Point3::new(-1.0, 1.0, 1.0),
            Vec3::new(-1.0, 0.0, 0.0)
        );

        // Back triangles
        let t7: Triangle = Triangle::new(
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(1.0, 1.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0)
        );
        let t8: Triangle = Triangle::new(
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(-1.0, 1.0, -1.0),
            Point3::new(1.0, 1.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0)
        );

        // Top triangles
        let t9: Triangle = Triangle::new(
            Point3::new(-1.0, 1.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(1.0, 1.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0)
        );
        let t10: Triangle = Triangle::new(
            Point3::new(-1.0, 1.0, 1.0),
            Point3::new(-1.0, 1.0, -1.0),
            Point3::new(1.0, 1.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0)
        );

        // Bot triangles
        let t11: Triangle = Triangle::new(
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, -1.0),
            Vec3::new(0.0, -1.0, 0.0)
        );
        let t12: Triangle = Triangle::new(
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(1.0, -1.0, -1.0),
            Vec3::new(0.0, -1.0, 0.0)
        );

        let cube: Mesh = Mesh::new_mesh(vec![
            t1,t2,t3,t4,t5,t6,t7,t8,t9,t10,t11,t12
            ]);
        return cube;
    }

    /// Check if a ray has hit any triangle in our mesh.
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

    /// Translate a mesh.
    pub fn translate(&mut self, d: Vec3) {
        for trig in self.triangles.iter_mut() {
            for point in trig.points.iter_mut() {
                *point = Point3::new(
                    point.x() + d.x(),
                    point.y() + d.y(),
                    point.z() + d.z());
            }
        }
    }

    /// Scale a mesh based on a constant.
    pub fn scale(&mut self, c: f32) {
        for trig in self.triangles.iter_mut() {
            for point in trig.points.iter_mut() {
                *point = Point3::new(
                    point.x() * c,
                    point.y() * c,
                    point.z() * c
                );
            }
        }
    }

    /// Rotate a mesh.
    pub fn rotate(&mut self, r: Vec3) {
        let theta_x = r.x().to_radians();
        let theta_y = r.y().to_radians();
        let theta_z = r.z().to_radians();

        for trig in self.triangles.iter_mut() {
            // First rotate the normal vector on x, y, z.

            // Rotate on x.
            trig.normal = Vec3::new(
                trig.normal.x(),
                trig.normal.y() * theta_x.cos() - trig.normal.z() * theta_x.sin(),
                trig.normal.y() * theta_x.sin() + trig.normal.z() * theta_x.cos()
            );

            // Rotate on y.
            trig.normal = Vec3::new(
                trig.normal.x() * theta_y.cos() + trig.normal.z() * theta_y.sin(),
                trig.normal.y(),
                -trig.normal.x() * theta_y.sin() + trig.normal.z() * theta_y.cos()
            );

            // Rotate on z.
            trig.normal = Vec3::new(
                trig.normal.x() * theta_z.cos() - trig.normal.y() * theta_z.sin(),
                trig.normal.x() * theta_z.sin() + trig.normal.y() * theta_z.cos(),
                trig.normal.z()
            );

            // Now rotate the individual points
            for point in trig.points.iter_mut() {
                // Rotate on x.
                *point = Point3::new(
                    point.x(),
                    point.y() * theta_x.cos() - point.z() * theta_x.sin(),
                    point.y() * theta_x.sin() + point.z() * theta_x.cos()
                );

                // Rotate on y.
                *point = Point3::new(
                    point.x() * theta_y.cos() + point.z() * theta_y.sin(),
                    point.y(),
                    -point.x() * theta_y.sin() + point.z() * theta_y.cos()
                );

                // Rotate on z.
                *point = Point3::new(
                    point.x() * theta_z.cos() - point.y() * theta_z.sin(),
                    point.x() * theta_z.sin() + point.y() * theta_z.cos(),
                    point.z()
                );
            }
        }

    }
}