use crate::{ray::Ray, hit::Hit, vec3::{Vec3, random_unit_vector, barycentric, unit_vector, reflect, dot, random_in_unit_sphere}};

#[derive(Clone, Debug)]
pub enum MaterialEnum {
    Diffuse(Diffuse),
    Metal(Metal)
}

impl Material for MaterialEnum {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            MaterialEnum::Diffuse(mat) => {
                return mat.scatter(r, hit, attenuation, scattered);
            }
            MaterialEnum::Metal(mat) => {
                return mat.scatter(r, hit, attenuation, scattered);
            }
        }
    }
    fn get_albedo(&self) -> Vec3 {
        match self {
            MaterialEnum::Diffuse(mat) => {
                return mat.get_albedo();
            },
            MaterialEnum::Metal(mat) => {
                return mat.get_albedo();
            },
        }
    }
}

pub trait Material {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
    fn get_albedo(&self) -> Vec3;
}

#[derive(Clone, Debug)]
pub struct Diffuse {
    pub albedo: Vec3
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Diffuse {
        return Diffuse { albedo };
    }
}

impl Material for Diffuse {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        if hit.triangle.smooth {
            // Calculate the barycentric coordinates
            let bary = barycentric(hit.clone());
            // Use the interpolated normal to randomly scatter the ray
            let mut scatter_direction = unit_vector(hit.triangle.normals[0] * bary.x + hit.triangle.normals[1] * bary.y + hit.triangle.normals[2] * bary.z) + random_unit_vector();
            if scatter_direction.near_zero() {
                // If we're close to zero, just set as the normal
                scatter_direction = hit.triangle.normal;
            }

            *scattered = Ray::new(hit.at, scatter_direction);
            *attenuation = self.albedo;

        } else {
            let mut scatter_direction = hit.triangle.normal + random_unit_vector();

            if scatter_direction.near_zero() {
                // If we're close to zero, just set as the normal
                scatter_direction = hit.triangle.normal;
            }

            *scattered = Ray::new(hit.at, scatter_direction);
            *attenuation = self.albedo;
        }
        return true;
    }

    fn get_albedo(&self) -> Vec3 {
        return self.albedo;
    }
}

/// Simple metal material
#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub smoothness: f64
}

impl Metal {
    pub fn new(albedo: Vec3, smoothness: f64) -> Metal {
        Metal {albedo, smoothness }
    }

}

impl Material for Metal {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        if hit.triangle.smooth {
            /*// Calculate the barycentric coordinates
            let bary = barycentric(hit.clone());
            // Use the interpolated normal to randomly scatter the ray
            let mut scatter_direction = unit_vector(hit.triangle.normals[0] * bary.x + hit.triangle.normals[1] * bary.y + hit.triangle.normals[2] * bary.z) + random_unit_vector();
            if scatter_direction.near_zero() {
                // If we're close to zero, just set as the normal
                scatter_direction = hit.triangle.normal;
            }

            *scattered = Ray::new(hit.at, scatter_direction);
            *attenuation = self.albedo;*/
            let bary = barycentric(hit.clone());
            let reflected = reflect(unit_vector(r.direction), unit_vector(hit.triangle.normals[0] * bary.x + hit.triangle.normals[1] * bary.y + hit.triangle.normals[2] * bary.z) + (random_in_unit_sphere() * self.smoothness));
            *scattered = Ray::new(hit.at, reflected);
            *attenuation = self.albedo;

            return dot(scattered.direction, hit.triangle.normals[0]) > 0.0 || dot(scattered.direction, hit.triangle.normals[1]) > 0.0 || dot(scattered.direction, hit.triangle.normals[2]) > 0.0;

        } else {
            let reflected = reflect(unit_vector(r.direction), hit.triangle.normal + (random_in_unit_sphere() * self.smoothness));
            *scattered = Ray::new(hit.at, reflected);
            *attenuation = self.albedo;

            return dot(scattered.direction, hit.triangle.normal) > 0.0;
        }
        return true;
    }
    fn get_albedo(&self) -> Vec3 {
        return self.albedo;
    }
}

/*#[derive(Copy, Clone, Debug)]
pub struct Diffuse {
    pub albedo: Vec3
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Diffuse {
        return Diffuse { albedo };
    }
}

impl Material for Diffuse {
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = self.albedo;
        return true;
    }
}*/
