use std::ops::Index;

use crate::vec3::{Point3, Vec3};

/// A struct that contains information for a triangle. Has an array of 
/// 3 poins, and a normal vector.
#[derive(Copy, Clone)]
pub struct Triangle {
    points: [Point3; 3],
    normal: Vec3,
}

impl Triangle {
    /// Create a new triangle.
    pub fn new(a: Point3, b: Point3, c: Point3, n: Vec3) -> Triangle {
        Triangle { points: [a,b,c], normal: n }
    }

    /// Get the triangles normal vector.
    pub fn normal(self) -> Vec3 {
        return self.normal;
    }
}

/// Get the point of a triangle based on the index. Iterate like an array.
impl Index<usize> for Triangle {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.points[index];
    }
}