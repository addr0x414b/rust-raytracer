use std::{fs::File, io::Write}; // Used to create/write to PPM file
use rand::Rng;

mod vec3;
use hit::Hit;
use vec3::{cross, unit_vector};

use crate::{vec3::{Vec3, Color, Point3, dot}, ray::Ray, mesh::Mesh, world::World};

mod ray;
mod triangle;
use crate::triangle::Triangle;

mod mesh;
mod world;
mod hit;

/// Write a color in PPM format to a PPM file using antialiasing.
/// # Arguments
/// * 'file' - Output PPM file, should already be initialized.
/// * 'color' - Color struct which contains x,y,z (rgb). 0 <= R,G,B <= 1.
fn write_color_aa(file: &mut File, color: Color, sample: u16) {
    let r: u16 = ((color.x() * (1.0 / sample as f32)).clamp(0.0, 0.999) * 255.0) as u16;
    let g: u16 = ((color.y() * (1.0 / sample as f32)).clamp(0.0, 0.999) * 255.0) as u16;
    let b: u16 = ((color.z() * (1.0 / sample as f32)).clamp(0.0, 0.999) * 255.0) as u16;
    //let r: u16 = (color.x() * 255.0) as u16;
    //let g: u16 = (color.y() * 255.0) as u16;
    //let b: u16 = (color.z() * 255.0) as u16;
    if r > 255 || g > 255 || b > 255 {
        panic!("write_color: R,G,B values are larger than 255");
    }
    file.write(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("Unable to write to file");
}

/// Write a color in PPM format to a PPM file with no antialiasing.
/// # Arguments
/// * 'file' - Output PPM file, should already be initialized.
/// * 'color' - Color struct which contains x,y,z (rgb). 0 <= R,G,B <= 1.
fn write_color(file: &mut File, color: Color) {
    let r: u16 = (color.x() * 255.0) as u16;
    let g: u16 = (color.y() * 255.0) as u16;
    let b: u16 = (color.z() * 255.0) as u16;
    if r > 255 || g > 255 || b > 255 {
        panic!("write_color: R,G,B values are larger than 255");
    }
    file.write(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("Unable to write to file");
}

/// Calculate color based on the ray sent from the origin and returns the color.
/// Currently linearly interpolating from pure white to light blue.
/// # Arguments
/// 'r' - Ray type, contains the origin and its direction
/// 'w' - World, contains all the meshes
fn ray_color(r: Ray, w: &World) -> Color {
    let hit: Hit = w.hit(r);
    if hit.t > 0.0 {
        let n = unit_vector(hit.triangle.normal());
        return Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0)*0.5;
    }
    let n = unit_vector(r.direction());
    let t = (n.y() + 1.0) * 0.5;
    return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + Color::new(0.5, 0.7, 1.0)*t;
}

fn main() {

    // Image properties
    /// PPM output aspect ratio. Used to calculate image height.
    /// # Description
    /// Stores the aspect ratio of our final image. Default 16 by 9.
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    /// PPM output image width
    /// # Description
    /// Stores the width of our final image in pixels.
    const IMAGE_WIDTH: u16 = 400;
    /// PPM output image height
    /// # Description
    /// Stores the height of our final image in pixels.
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

    /// Enable/disable antialiasing.
    const ANTIALIASING: bool = false;
    /// How many times we sample for antialiasing.
    const SAMPLES_PER_PIXEL: u16 = 500;

    // Camera properties
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
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

    let mut cube: Mesh = Mesh::new_cube();
    cube.rotate(Vec3::new(0.0, 30.0, 0.0));
    cube.translate(Vec3::new(0.0, -0.5, -4.0));

    let mut plane: Mesh = Mesh::new_plane();
    plane.scale(20.0);
    plane.translate(Vec3::new(0.0, -1.5, -4.0));

    let mut world: World = World::new();

    world.add(plane);
    world.add(cube);

    // Loop over every single pixel in our image
    for y in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT-y);
        for x in 0..IMAGE_WIDTH {
            if ANTIALIASING {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for sample in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let u: f32 = ((x) as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                    let v: f32 = 1.0 - ((y as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32);

                    let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);
                    color = color + ray_color(r, &world);

                }
                // Write our r,g,b values to every single pixel
                write_color_aa(&mut output_file, color, SAMPLES_PER_PIXEL);
            } else {
                    let u: f32 = ((x) as f32) / (IMAGE_WIDTH - 1) as f32;
                    let v: f32 = 1.0 - ((y as f32) / (IMAGE_HEIGHT - 1) as f32);

                    let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);
                    let color = ray_color(r, &world);

                    // Write our r,g,b values to every single pixel
                    write_color(&mut output_file, color);

            }
        }
    }
}