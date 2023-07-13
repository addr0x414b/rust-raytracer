// Things I didn't write
use std::{fs::File, io::Write};
use material::Material;
// Used to create/write to PPM file
use rand::Rng; // Used to generate random numbers for sampling 

// Things I wrote
mod vec3;
mod ray;
mod triangle;
mod mesh;
mod world;
mod hit;
mod material;

use hit::Hit;
use vec3::{Vec3, Color, Point3, unit_vector};
use ray::Ray;
use mesh::{Mesh, MaterialEnum};
use world::World;

use crate::material::{Diffuse, Metal};

/// Determine which drawing mode we should use
/// * 'Normals' - Draw only the normal colors of objects
/// * 'Sampling' - Draw with sampling enabled
enum DrawingMode {
    Normals,
    Sampling,
}

/// Calculate color based on ray and whatever the ray hits
/// # Arguments
/// * 'r' - Ray type, contains the origin and its direction
/// * 'w' - World reference, contains all the meshes
/// * 'depth' - How many bounces a ray can take
/// * 'mode' - The drawing mode
/// # Return
/// * Color based on the ray 
fn ray_color(r: Ray, w: &World, depth: u16, mode: DrawingMode) -> Color {
    match mode {
        // Check which drawing mode is enabled
        DrawingMode::Sampling => {
            if depth <= 0 { // If we've reached our last bounce, no more color to add
                return Color::new(0.0, 0.0, 0.0);
            }
            let hit: Hit = w.hit(r); // Check if our ray has hit anything. T will be positive
            if hit.t > 0.0 {
                // Scattered gives us the new ray based on the material of the object. For example,
                // if we hit a perfectly smooth object, the ray bounce will be a perfect bounce away,
                // so the ray will "scatter" perfectly. Whereas for diffuse, it's randomly scattered 
                // due to the surface not being smooth
                let mut scattered: Ray = Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0));
                // When we hit an object, attenuation is set as the color of the object. We then
                // multiply by the recursive bounce. This allows the color to take in account all the objects
                // we bounce off, which is similar to what light does in real life. Light absorption
                let mut attenuation: Color = Color::new(0.0, 0.0, 0.0);

                // Check the material of the object we hit. I don't like repeating code, need to find a way
                // to make this cleaner... but it works for now
                match hit.material {
                    MaterialEnum::Diffuse(ref diffuse) => {
                        // The scatter function returns a bool. We want to check if we've scattered properly
                        // We set the values of attenuation and scattered in that function call
                        if diffuse.scatter(r, hit.clone(), &mut attenuation, &mut scattered) {
                            // Recursively multiply the final color by all the bounces of objects
                            return attenuation * ray_color(scattered, w, depth-1, mode);
                        }
                        // If we don't scatter, we just return black
                        return Color::new(0.0,0.0,0.0);
                    },
                    MaterialEnum::Metal(ref metal) => {
                        if metal.scatter(r, hit.clone(), &mut attenuation, &mut scattered) {
                            return attenuation * ray_color(scattered, w, depth-1, mode);
                        }
                        return Color::new(0.0,0.0,0.0);
                    }
                }
            }
        },
        DrawingMode::Normals => {
            let hit: Hit = w.hit(r);
            if hit.t > 0.0 {
                let n = hit.triangle.normal;
                // Standard way of calculating color based on a normal vector
                return Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0)*0.5;
            }

        }
    }
    // This code generates the blueish gradient background
    let n = r.direction;
    let t = (n.y() + 1.0) * 0.5;
    // Typical interpolation
    return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + Color::new(0.5, 0.7, 1.0)*t;
}


/// Write a color in PPM format to a PPM file
/// # Arguments
/// * 'file' - Output PPM file, should already be initialized
/// * 'color' - Color struct which contains x,y,z (rgb). 0 <= R,G,B <= 1
/// * 'sample' - The number of samples for the ray
/// * 'mode' - The drawing mode 
fn write_color(file: &mut File, color: Color, samples: u16, mode: DrawingMode) {
    let r: u16;
    let g: u16;
    let b: u16;
    match mode {
        DrawingMode::Sampling=> {
            // Divide 1.0 by samples to gamma correct
            r = ((color.x() * (1.0 / samples as f32)).sqrt().clamp(0.0, 0.999) * 255.0) as u16;
            g = ((color.y() * (1.0 / samples as f32)).sqrt().clamp(0.0, 0.999) * 255.0) as u16;
            b = ((color.z() * (1.0 / samples as f32)).sqrt().clamp(0.0, 0.999) * 255.0) as u16;
        },
        DrawingMode::Normals => {
            // Colors: 0 <= c <= 1, multiply by 255 to convert to 0 <= c <= 255 
            r = (color.x() * 255.0) as u16;
            g = (color.y() * 255.0) as u16;
            b = (color.z() * 255.0) as u16;
        }
    }
    if r > 255 || g > 255 || b > 255 {
        panic!("write_color: R,G,B values are larger than 255");
    }
    file.write(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("Unable to write to file");
}

fn main() {

    // Image properties
    /// PPM output aspect ratio. Used to calculate image height
    /// # Description
    /// * The aspect ratio of our final image. Default 16 by 9
    const ASPECT_RATIO: f32 = 16.0 / 9.0;

    /// PPM output image width
    /// # Description
    /// * The width of our final image in pixels
    const IMAGE_WIDTH: u16 = 1920;

    /// PPM output image height
    /// # Description
    /// * The height of our final image in pixels
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

    /// The drawing mode from the enum
    /// # Description
    /// * 'Normals' - Draw only the normal colors of objects
    /// * 'Sampling' - Draw with sampling enabled
    const DRAWING_MODE: DrawingMode = DrawingMode::Sampling;

    /// Pixel samples
    /// # Description
    /// * How many times we sample per pixel
    const SAMPLES_PER_PIXEL: u16 = 10;

    /// Ray bounces
    /// # Description
    /// How many times a ray bounces
    const MAX_DEPTH: u16 = 50;

    // Camera properties
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    // Next 3 variables define our camera space
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    // Can use the lower left corner for our calculations
    let lower_left_corner: Vec3 = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Create a PPM file which will store our raytraced image
    let mut output_file: File = File::create("output.ppm")
        .expect("Unable to create output.ppm file");

    // Initialize properties of the output file. P3 says the colors will be in
    // ASCII, the next two values are the width of the image in pixels and the
    // height of the image in pixels. The last 255 is the largest value a color
    // can be... ie (255, 0, 0) would be max red, 0 green and 0 blue
    output_file.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .expect("Unable to initiate output.ppm properties");

    // Just create some default objects to play around with
    let mut cube: Mesh = Mesh::new_cube();
    cube.rotate(Vec3::new(0.0, 30.0, 0.0));
    cube.translate(Vec3::new(0.0, -0.5, -4.0));
    cube.material = MaterialEnum::Diffuse(Diffuse::new(Color::new(0.7, 0.3, 0.3)));

    let mut cube2: Mesh = Mesh::new_cube();
    cube2.translate(Vec3::new(3.0, -0.5, -3.0));
    cube2.material = MaterialEnum::Metal(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let mut cube3: Mesh = Mesh::new_cube();
    cube3.translate(Vec3::new(-3.0, -0.5, -3.0));
    cube3.material = MaterialEnum::Metal(Metal::new(Color::new(0.8, 0.8, 0.8)));

    let mut plane: Mesh = Mesh::new_plane();
    plane.scale(20.0);
    plane.rotate(Vec3::new(0.0, 0.0, 0.0));
    plane.translate(Vec3::new(0.0, -2.0, -4.0));
    plane.material = MaterialEnum::Diffuse(Diffuse::new(Color::new(0.8, 0.8, 0.0)));

    // The world stores all the meshes that we can draw
    let mut world: World = World::new();

    world.add(plane);
    world.add(cube);
    world.add(cube2);
    world.add(cube3);

    // Loop over every single pixel in our image
    for y in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT-y);
        for x in 0..IMAGE_WIDTH {
            match DRAWING_MODE {
                DrawingMode::Normals => {
                    // Transform from pixel space (x,y) to 'camera' space (u,v)
                    let u: f32 = ((x) as f32) / (IMAGE_WIDTH - 1) as f32;
                    // 1 minus because I want to flip the image. By default it's upside down
                    let v: f32 = 1.0 - ((y as f32) / (IMAGE_HEIGHT - 1) as f32);

                    // Create a new ray that starts from the camera origin and points towards the world
                    // Must convert the direction to a unit vector, otherwise StRaNgE things happen
                    let r = Ray::new(origin, unit_vector(lower_left_corner + (horizontal*u) + (vertical*v) - origin));
                    let color = ray_color(r, &world, MAX_DEPTH, DRAWING_MODE); // Calculate color based on what the ray hits

                    // Write our r,g,b values to every single pixel
                    write_color(&mut output_file, color, 0, DRAWING_MODE);
                },
                DrawingMode::Sampling => {
                    // Because we're sampling, we want to average out our color, so make it mutable 
                    let mut color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..SAMPLES_PER_PIXEL { // Loop for however many samples we want
                        let mut rng = rand::thread_rng(); // Need to randomly pick samples around our pixel
                        let u: f32 = ((x) as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                        let v: f32 = 1.0 - ((y as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32);

                        let r = Ray::new(origin, unit_vector(lower_left_corner + (horizontal*u) + (vertical*v) - origin));
                        color = color + ray_color(r, &world, MAX_DEPTH, DRAWING_MODE);

                    }
                    // Write our r,g,b values to every single pixel
                    write_color(&mut output_file, color, SAMPLES_PER_PIXEL, DRAWING_MODE);
                }
            }
        }
    }
}