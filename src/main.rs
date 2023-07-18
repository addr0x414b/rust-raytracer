// Things I didn't write
use std::{fs::File, io::Write};
use rand::Rng;

// Things I wrote
mod vec3;
mod triangle;
mod mesh;
mod ray;
mod world;
mod hit;
mod material;

use vec3::{Vec3, barycentric, unit_vector};
use mesh::load_mesh;
use ray::Ray;
use world::World;
use material::{Material, MaterialEnum, Diffuse, Metal};

/// Determine which drawing mode to use
/// * 'Colors' - Draw only the colors of the objects 
/// * 'Normals' - Draw only the normals of the objects
/// * 'Samples' - Draw the final image with sampling
#[derive(Copy, Clone)]
enum DrawingMode {
    Colors,
    Normals,
    Samples
}

/// Calculate color based on the ray and whatever it hits
/// # Arguments
/// * 'r' - Ray to cast
/// * 'w' - World which contains all objects 
/// * 'depth' - Number of bounces a ray can have
/// * 'mode' - Drawing mode
/// # Returns
/// * Vec3 which contains r,g,b values in the x,y,z position of the vector
fn ray_color(r: Ray, w: World, depth: u32, mode: DrawingMode) -> Vec3 {
    
    // Check if our ray hits any object
    // Hit will contain details about the object the ray hit
    let hit = w.hit(r);

    // Match the drawing mode
    match mode {
        DrawingMode::Colors => {

            // Hit.t will be > 0 if the ray actually hit something
            if hit.t > 0.0 {

                // Simply return the color of what the ray hit
                return hit.material.get_albedo();
            }
        },
        DrawingMode::Normals => {
            if hit.t > 0.0 {
                // N will store the normal of what we hit
                let n: Vec3;

                // If the mesh is smooth shaded, we need to calculate the interpolated normal
                if hit.triangle.smooth {

                    // Calculate the barycentric coordinates 
                    let bary = barycentric(hit.clone());

                    // Calculate the interpolated normal
                    n = unit_vector(hit.triangle.normals[0] * bary.x + hit.triangle.normals[1] * bary.y + hit.triangle.normals[2] * bary.z);
                } else {

                    // Mesh isn't smooth shaded, simply return its single normal
                    n = hit.triangle.normal;
                }

                // Calculate color based on the normal
                return Vec3::new(n.x+1.0, n.y+1.0, n.z+1.0) * 0.5;
            }
        },
        DrawingMode::Samples => {

            // Samples mode recursively calls ray_color
            // Quit recursively calling if we've bounced our last bounce
            if depth <= 0 {
                return Vec3::new(0.0, 0.0, 0.0);
            }
            if hit.t > 0.0 {

                // Will store the new ray, i.e. we bounce off the object and have a new ray based on the bounce
                let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

                // Store the current color of whatever the ray bounces off
                let mut attenuation = Vec3::new(0.0, 0.0, 0.0);

                // Make sure we correctly scatter based on the objects material
                if hit.material.scatter(r, hit.clone(), &mut attenuation, &mut scattered) {

                    // Recursively call, multiplying the current color
                    return attenuation * ray_color(scattered, w.clone(), depth-1, mode.clone());
                }
            }
        }
    }

    // This code generates the blueish gradient background
    let n = r.direction;
    let t = (n.y + 1.0) * 0.5;

    // Typical interpolation
    return (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + Vec3::new(0.5, 0.7, 1.0)*t;
}


/// Write a color to the output file
/// # Arguments
/// * 'file' - PPM file we write to
/// * 'color' - Color which we wish to write
/// * 'samples' - Number of samples 
/// * 'mode' - Drawing mode
fn write_color(file: &mut File, color: Vec3, samples: u32, mode: DrawingMode) {
    let r: u32;
    let g: u32;
    let b: u32;
    match mode {
        DrawingMode::Colors | DrawingMode::Normals => {

            // If we're drawing colors/normals, simply multiply by 255
            // Input color is 0-1, so multiply by 255 to make it in a range of 0-255
            r = (color.x * 255.0) as u32;
            g = (color.y * 255.0) as u32;
            b = (color.z * 255.0) as u32;
        },
        DrawingMode::Samples => {

            // Perform gamma correction
            r = ((color.x * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;
            g = ((color.y * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;
            b = ((color.z * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;

        }
    }
    if r > 255 || g > 255 || b > 255 {
        panic!("Color value out of range");
    }

    file.write(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("Unable to write to output file");
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const DRAWING_MODE: DrawingMode = DrawingMode::Samples;

    /// How many pixels we sample for colors and antialiasing 
    const SAMPLES : u32 = 3;

    /// Number of bounces a ray can perform
    const MAX_DEPTH: u32 = 5;

    let mut output_file = File::create("output.ppm")
        .expect("Failed to create PPM file");

    output_file.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .expect("Failed to write to PPM file");

    // Viewport properties
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;

    // Camera properties
    let focal_length = 5.0;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Default scene
    // Floor object
    let mut floor = load_mesh("models/plane.obj", false);
    floor.scale(4.0);
    floor.rotate(Vec3::new(0.0, 0.0, 0.0));
    floor.translate(Vec3::new(0.0, -1.4, -10.0));
    floor.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.89, 0.4, 0.4), 0.0));

    // Cube object
    let mut cube = load_mesh("models/cube.obj", false);
    cube.scale(1.0);
    cube.rotate(Vec3::new(0.0, 10.0, 0.0));
    cube.translate(Vec3::new(0.0, -0.4, -12.0));
    cube.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.8, 0.8, 0.4)));

    // Default world
    let mut world = World::new();

    // Add objects to the world
    world.add(floor);
    world.add(cube);


    // Loop through our image
    for y in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", y+1);
        for x in 0..IMAGE_WIDTH {
            match DRAWING_MODE {
                DrawingMode::Colors | DrawingMode::Normals => {
                    let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
                    let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;

                    // Calculate the ray based on the pixel we are on
                    let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);

                    // Send over the ray and world and figure out the color we should draw for this pixel
                    let color = ray_color(r, world.clone(), MAX_DEPTH, DRAWING_MODE);

                    write_color(&mut output_file, color, 1, DRAWING_MODE);
                },
                DrawingMode::Samples => {
                    let mut color = Vec3::new(0.0, 0.0, 0.0);

                    // Loop for however many samples we want to take
                    for _ in 0..SAMPLES {
                        
                        // Need random number generator from 0-1
                        let mut rng = rand::thread_rng();

                        // Calculate u&v based on our random samples
                        let u: f64 = ((x) as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                        let v: f64 = (y as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                        let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);

                        // Add to the color for each sample, essentially creating an average color
                        color = color + ray_color(r, world.clone(), MAX_DEPTH, DRAWING_MODE);
                    }
                    write_color(&mut output_file, color, SAMPLES, DRAWING_MODE);
                }
            }
        }
    }
}