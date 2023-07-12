use crate::{ray::Ray, hit::Hit, vec3::{Color, random_unit_vector, reflect, unit_vector, dot}};

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
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal {color }
    }
}

impl Material for Diffuse {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit.triangle.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.triangle.normal;
        }
        *scattered = Ray::new(hit.at, scatter_direction);
        *attenuation = self.color;
        return true;
    }
}

impl Material for Metal {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r.direction), hit.triangle.normal);
        *scattered = Ray::new(hit.at, reflected);
        *attenuation = self.color;
        return dot(scattered.direction, hit.triangle.normal) > 0.0;
    }
}