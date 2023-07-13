use crate::{ray::Ray, hit::Hit, vec3::{Color, random_unit_vector, reflect, unit_vector, dot, random_in_unit_sphere, Vec3}};

/// A material trait that we use for all materials
pub trait Material {
    /// Scatter an incoming ray based on what material it is
    /// # Arguments
    /// * 'r' - The incoming ray
    /// * 'hit' - The triangle we hit
    /// * 'attenuation' - The amount of reduction in the ray's color after scattering
    /// * 'scattered' - The final scattered ray based on the bounce
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

/// Simple diffuse material
#[derive(Copy, Clone)]
pub struct Diffuse {
    pub color: Color,
}

impl Diffuse {
    pub fn new(color: Color) -> Diffuse {
        Diffuse { color }
    }
}

/// Simple metal material
#[derive(Copy, Clone)]
pub struct Metal {
    pub color: Color,
    pub smoothness: f32
}

impl Metal {
    pub fn new(color: Color, smoothness: f32) -> Metal {
        Metal {color, smoothness }
    }
}

/// Implementation for diffuse ray scattering
impl Material for Diffuse {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction: Vec3;

        if hit.triangle.smooth {
            let v0 = hit.triangle.points[1] - hit.triangle.points[0];
            let v1 = hit.triangle.points[2] - hit.triangle.points[0];
            let v2 = hit.at - hit.triangle.points[0];

            let d00 = dot(v0, v0);
            let d01 = dot(v0, v1);
            let d11 = dot(v1, v1);
            let d20 = dot(v2, v0);
            let d21 = dot(v2, v1);

            let denom = d00 * d11 - d01 * d01;

            let v = (d11 * d20 - d01 * d21) / denom;
            let w = (d00 * d21 - d01 * d20) / denom;
            let u = 1.0 - v - w;
            scatter_direction = unit_vector(hit.triangle.normals[0] * u + hit.triangle.normals[1] * v + hit.triangle.normals[2] * w) + random_unit_vector();
        } else {
            scatter_direction = hit.triangle.normal + random_unit_vector();
        }

        if scatter_direction.near_zero() {
            // If we're close to zero, just set as the normal
            scatter_direction = hit.triangle.normal;
        }
        // Set scattered to this new ray. This is the bounce we made, or the scattering ray
        *scattered = Ray::new(hit.at, scatter_direction);
        *attenuation = self.color;
        return true; // Default return true
    }
}

/// Implementation for metal ray scattering
impl Material for Metal {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected: Vec3;

        if hit.triangle.smooth {
            let v0 = hit.triangle.points[1] - hit.triangle.points[0];
            let v1 = hit.triangle.points[2] - hit.triangle.points[0];
            let v2 = hit.at - hit.triangle.points[0];

            let d00 = dot(v0, v0);
            let d01 = dot(v0, v1);
            let d11 = dot(v1, v1);
            let d20 = dot(v2, v0);
            let d21 = dot(v2, v1);

            let denom = d00 * d11 - d01 * d01;

            let v = (d11 * d20 - d01 * d21) / denom;
            let w = (d00 * d21 - d01 * d20) / denom;
            let u = 1.0 - v - w;
            reflected = reflect(unit_vector(r.direction), unit_vector(hit.triangle.normals[0] * u + hit.triangle.normals[1] * v + hit.triangle.normals[2] * w));
        } else {
            reflected = reflect(unit_vector(r.direction), hit.triangle.normal);
        }


        // Find the direction of the ray based on a smooth reflection
        //let reflected = reflect(unit_vector(r.direction), hit.triangle.normal);
        // New ray bounce. Use the smoothness value to determine how 'fuzzy' the metal material is
        *scattered = Ray::new(hit.at, reflected + random_in_unit_sphere() * self.smoothness);
        *attenuation = self.color;
        // Only return true if the new ray is in the same general direction as the triangle normal
        return dot(scattered.direction, hit.triangle.normal) > 0.0;
    }
}