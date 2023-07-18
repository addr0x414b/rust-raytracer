use crate::{ray::Ray, hit::Hit, vec3::{Vec3, random_unit_vector, barycentric, unit_vector, reflect, dot, random_in_unit_sphere}};

/// Store all the different types of materials
#[derive(Clone, Debug)]
pub enum MaterialEnum {
    Diffuse(Diffuse),
    Metal(Metal)
}

/// Contains functions every material needs to be able to perform
/// # Functions
/// * 'scatter' - Tells the program how the ray should scatter based on the material 
/// * 'get_albedo' - Return the objects albedo color
pub trait Material {
    /// Determine how the ray will bounce off the object based on its material
    /// # Arguments
    /// * 'r' - The incoming ray
    /// * 'hit' - Information about what we hit
    /// * 'attenuation' - Will store the hit color
    /// * 'scattered' - Will store the new bounced ray
    /// # Returns
    /// * True or false based on the individual scatter implementation
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;

    /// Return the color of the object
    /// # Returns
    /// * Vec3 containing r,g,b values of the object in the x,y,z position
    fn get_albedo(&self) -> Vec3;
}

impl Material for MaterialEnum {

    // Not sure if there's a better way to do this...
    // Need to call the various functions of the material based on which material it actually is

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

/// Simple diffuse material
#[derive(Clone, Debug)]
pub struct Diffuse {
    /// The objects albedo color
    pub albedo: Vec3
}

impl Diffuse {
    /// Create a new diffuse material
    /// # Arguments
    /// * 'albedo' - Desired color
    pub fn new(albedo: Vec3) -> Diffuse {
        return Diffuse { albedo };
    }
}

impl Material for Diffuse {

    // Scatter function for an object with a diffuse material
    fn scatter(&self, _r: Ray, hit: Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {

        let mut scatter_direction: Vec3;

        // If the object is smooth shaded
        if hit.triangle.smooth {

            // Calculate the barycentric coordinates
            let bary = barycentric(hit.clone());

            // Create a new ray that's scattered
            // The ray bounces based on the interpolated normal and a random unit vector, which aims to simulate diffuse's rough look
            scatter_direction = unit_vector(hit.triangle.normals[0] * bary.x + hit.triangle.normals[1] * bary.y + hit.triangle.normals[2] * bary.z) + random_unit_vector();

            if scatter_direction.near_zero() {
                // If we're close to zero, just set as the normal
                scatter_direction = hit.triangle.normal;
            }

            // Set the current scattered ray based on the location the ray hit and the new direction
            *scattered = Ray::new(hit.at, scatter_direction);
            *attenuation = self.albedo; // Current objects color

        } else { // Not smooth shaded

            // Use the single normal and bounce
            scatter_direction = hit.triangle.normal + random_unit_vector();

            if scatter_direction.near_zero() {
                scatter_direction = hit.triangle.normal;
            }

            *scattered = Ray::new(hit.at, scatter_direction);
            *attenuation = self.albedo;
        }
        return true;
    }

    // Simply return the albedo color
    fn get_albedo(&self) -> Vec3 {
        return self.albedo;
    }
}

/// Simple metal material
#[derive(Copy, Clone, Debug)]
pub struct Metal {
    /// Albedo color
    pub albedo: Vec3,
    /// How smooth the metal is
    pub smoothness: f64
}

impl Metal {
    /// Create a new metal material
    /// # Arguments
    /// * 'albedo' - Desired color
    /// * 'smoothness' - How smooth metal is (0.0 is mirror, 1.0 not smooth at all)
    pub fn new(albedo: Vec3, smoothness: f64) -> Metal {
        Metal {albedo, smoothness }
    }

}

impl Material for Metal {

    // Scatter for a metal material
    fn scatter(&self, r: Ray, hit: Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {

        // Will contain the reflected direction
        let reflected: Vec3;

        // If the object is smooth shaded
        if hit.triangle.smooth {

            // Calculate barycentric coordinates
            let bary = barycentric(hit.clone());

            // Call reflect function based on the input ray direction and the interpolated normal
            // Multiply by a random in unit sphere and smoothness to change how smooth the reflection is
            reflected = reflect(unit_vector(r.direction), unit_vector(hit.triangle.normals[0] * bary.x + hit.triangle.normals[1] * bary.y + hit.triangle.normals[2] * bary.z) + (random_in_unit_sphere() * self.smoothness));

            // Set the new scattered direction based on the reflection
            *scattered = Ray::new(hit.at, reflected);
            *attenuation = self.albedo;

            // Make sure the scattered direction is in a similar direction as the normals of each vertex
            return dot(scattered.direction, hit.triangle.normals[0]) > 0.0 || dot(scattered.direction, hit.triangle.normals[1]) > 0.0 || dot(scattered.direction, hit.triangle.normals[2]) > 0.0;

        } else {
            reflected = reflect(unit_vector(r.direction), hit.triangle.normal + (random_in_unit_sphere() * self.smoothness));
            *scattered = Ray::new(hit.at, reflected);
            *attenuation = self.albedo;

            // Make sure the direction is similar to the triangle's normal
            return dot(scattered.direction, hit.triangle.normal) > 0.0;
        }
    }

    fn get_albedo(&self) -> Vec3 {
        return self.albedo;
    }
}