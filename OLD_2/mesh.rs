use std::{fs::File, io::{BufReader, BufRead}};

use crate::{triangle::{Triangle, self}, ray::Ray, hit::Hit, vec3::{Vec3, Point3, unit_vector, Color}, material::{Diffuse, Metal}};

/// Material enum. Contains all the different possible materials for a mesh
#[derive(Clone)]
pub enum MaterialEnum {
    Diffuse(Diffuse),
    Metal(Metal)
}

/// A mesh that is rendered in the world
#[derive(Clone)]
pub struct Mesh {
    /// A mesh consists of many triangles
    pub triangles: Vec<Triangle>,
    /// The mesh's material
    pub material: MaterialEnum,
}

impl Mesh {
    /// Create an empty mesh
    pub fn new() -> Mesh {
        Mesh { triangles: Vec::new(), material: MaterialEnum::Diffuse(Diffuse::new(Color::new(0.5, 0.5, 0.5))) }
    }

    /// Create a mesh with an already established Vec of triangles
    pub fn new_mesh(trigs: Vec<Triangle>) -> Mesh {
        Mesh { triangles: trigs, material: MaterialEnum::Diffuse(Diffuse::new(Color::new(0.5, 0.5, 0.5))) }
    }

    /// Create a default plane mesh
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

    /// Check if a ray has hit any triangle in our mesh
    pub fn hit(&self, r: Ray) -> Hit {

        // We want to store the closest hit triangle so we only draw those
        let mut closest_hit: Hit = Hit::new();

        // Loop through every triangle within 'us'
        for trig in self.triangles.iter() {
            let mut hit: Hit = trig.hit(r); // Check if the ray has hit any of our triangles
            hit.material = self.material.clone();
            if hit.t > 0.0 {
                // Check if the hit triangle is closer than the current closest
                if hit.at.z() > closest_hit.at.z() {
                    closest_hit = hit;
                }
            }
        }
        // Return the triangle that was hit that is also the closest
        // Note: if we don't hit a triangle, the default z value of hit's 'at' is -5000000,
        // so we won't draw anything
        return closest_hit;
    }

    /// Translate a mesh
    /// # Arguments
    /// * 'd' - The (x,y,z) of d translate in the x,y,z directions
    pub fn translate(&mut self, d: Vec3) {
        // Loop over each triangle in the mesh and simply add x,y,z to the points to translate
        for trig in self.triangles.iter_mut() {
            for point in trig.points.iter_mut() {
                *point = Point3::new(
                    point.x() + d.x(),
                    point.y() + d.y(),
                    point.z() + d.z());
            }
        }
    }

    /// Scale a mesh based on a constant
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

    /// Rotate a mesh
    /// # Arguments
    /// * 'r' - Vec3 in degrees NOT radians
    pub fn rotate(&mut self, r: Vec3) {
        // Must convert to radians
        let theta_x = r.x().to_radians();
        let theta_y = r.y().to_radians();
        let theta_z = r.z().to_radians();

        // Need to rotate each triangle in the mesh
        for trig in self.triangles.iter_mut() {
            // First, let's rotate the normal 
            // Rotate on x
            trig.normal = Vec3::new(
                trig.normal.x(),
                trig.normal.y() * theta_x.cos() - trig.normal.z() * theta_x.sin(),
                trig.normal.y() * theta_x.sin() + trig.normal.z() * theta_x.cos()
            );
            trig.normals[0] = Vec3::new(
                trig.normals[0].x(),
                trig.normals[0].y() * theta_x.cos() - trig.normals[0].z() * theta_x.sin(),
                trig.normals[0].y() * theta_x.sin() + trig.normals[0].z() * theta_x.cos()
            );
            trig.normals[1] = Vec3::new(
                trig.normals[1].x(),
                trig.normals[1].y() * theta_x.cos() - trig.normals[1].z() * theta_x.sin(),
                trig.normals[1].y() * theta_x.sin() + trig.normals[1].z() * theta_x.cos()
            );
            trig.normals[2] = Vec3::new(
                trig.normals[2].x(),
                trig.normals[2].y() * theta_x.cos() - trig.normals[2].z() * theta_x.sin(),
                trig.normals[2].y() * theta_x.sin() + trig.normals[2].z() * theta_x.cos()
            );
            trig.normal = unit_vector(trig.normal);
            trig.normals[0] = unit_vector(trig.normals[0]);
            trig.normals[1] = unit_vector(trig.normals[1]);
            trig.normals[2] = unit_vector(trig.normals[2]);

            // Rotate on y
            trig.normal = Vec3::new(
                trig.normal.x() * theta_y.cos() + trig.normal.z() * theta_y.sin(),
                trig.normal.y(),
                -trig.normal.x() * theta_y.sin() + trig.normal.z() * theta_y.cos()
            );
            trig.normals[0] = Vec3::new(
                trig.normals[0].x() * theta_y.cos() + trig.normals[0].z() * theta_y.sin(),
                trig.normals[0].y(),
                -trig.normals[0].x() * theta_y.sin() + trig.normals[0].z() * theta_y.cos()
            );
            trig.normals[1] = Vec3::new(
                trig.normals[1].x() * theta_y.cos() + trig.normals[1].z() * theta_y.sin(),
                trig.normals[1].y(),
                -trig.normals[1].x() * theta_y.sin() + trig.normals[1].z() * theta_y.cos()
            );
            trig.normals[2] = Vec3::new(
                trig.normals[2].x() * theta_y.cos() + trig.normals[2].z() * theta_y.sin(),
                trig.normals[2].y(),
                -trig.normals[2].x() * theta_y.sin() + trig.normals[2].z() * theta_y.cos()
            );
            trig.normal = unit_vector(trig.normal);
            trig.normals[0] = unit_vector(trig.normals[0]);
            trig.normals[1] = unit_vector(trig.normals[1]);
            trig.normals[2] = unit_vector(trig.normals[2]);

            // Rotate on z
            trig.normal = Vec3::new(
                trig.normal.x() * theta_z.cos() - trig.normal.y() * theta_z.sin(),
                trig.normal.x() * theta_z.sin() + trig.normal.y() * theta_z.cos(),
                trig.normal.z()
            );
            trig.normals[0] = Vec3::new(
                trig.normals[0].x() * theta_z.cos() - trig.normals[0].y() * theta_z.sin(),
                trig.normals[0].x() * theta_z.sin() + trig.normals[0].y() * theta_z.cos(),
                trig.normals[0].z()
            );
            trig.normals[1] = Vec3::new(
                trig.normals[1].x() * theta_z.cos() - trig.normals[1].y() * theta_z.sin(),
                trig.normals[1].x() * theta_z.sin() + trig.normals[1].y() * theta_z.cos(),
                trig.normals[1].z()
            );
            trig.normals[2] = Vec3::new(
                trig.normals[2].x() * theta_z.cos() - trig.normals[2].y() * theta_z.sin(),
                trig.normals[2].x() * theta_z.sin() + trig.normals[2].y() * theta_z.cos(),
                trig.normals[2].z()
            );

            // We MUST normalize the normal after rotating
            trig.normal = unit_vector(trig.normal);
            trig.normals[0] = unit_vector(trig.normals[0]);
            trig.normals[1] = unit_vector(trig.normals[1]);
            trig.normals[2] = unit_vector(trig.normals[2]);

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

/// Load an OBJ mesh given a file path
pub fn load_mesh(path: &str, smooth: bool) -> Mesh {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let words: Vec<&str> = line.split_whitespace().collect();


        if words[0] == "v" {
            vertices.push([
                words[1].parse().unwrap(),
                words[2].parse().unwrap(),
                words[3].parse().unwrap()
            ]);
        } else if words[0] == "vn" {
            normals.push([
                words[1].parse().unwrap(),
                words[2].parse().unwrap(),
                words[3].parse().unwrap()
            ]);
        } else if words[0] == "f" {
            let v1: Vec<&str> = words[1].split('/').collect();
            let v2: Vec<&str> = words[2].split('/').collect();
            let v3: Vec<&str> = words[3].split('/').collect();

            let p1: usize = v1[0].parse().unwrap();
            let n1: usize = v1[2].parse().unwrap();

            let p2: usize = v2[0].parse().unwrap();
            let n2: usize = v2[2].parse().unwrap();

            let p3: usize = v3[0].parse().unwrap();
            let n3: usize = v3[2].parse().unwrap();

            let mut trig = Triangle::new(
                Point3::new(vertices[p1-1][0], vertices[p1-1][1], vertices[p1-1][2],),
                Point3::new(vertices[p2-1][0], vertices[p2-1][1], vertices[p2-1][2],),
                Point3::new(vertices[p3-1][0], vertices[p3-1][1], vertices[p3-1][2],),
                Vec3::new(normals[n1-1][0], normals[n1-1][1], normals[n1-1][2]),
            );

            if smooth {
                trig.smooth = true;

                trig.normals = [
                    Vec3::new(normals[n1-1][0], normals[n1-1][1], normals[n1-1][2]),
                    Vec3::new(normals[n2-1][0], normals[n2-1][1], normals[n2-1][2]),
                    Vec3::new(normals[n3-1][0], normals[n3-1][1], normals[n3-1][2]),
                ];
            }

            triangles.push(trig);
        }
    }
    return Mesh::new_mesh(triangles);
}