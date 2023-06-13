use std::fs::File;
use std::io::Write;

mod vec3;
mod color;
mod ray;

use crate::vec3::unit_vector;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::color::write_color;

// Calculate the gradient background based on the ray 
fn ray_color(r: Ray) -> Color {
    // Normalize the direction
    let unit_direction: Vec3 = unit_vector(r.direction());
    // Calculate the intensity for the Y of the direction. It's -1 to 1 and we need 0 to 1
    let t: f32 = (unit_direction.y() + 1.0) * 0.5;
    // Calculate the color based on the vertical position of the ray
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {

    // Initialize image properties
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

    // Initialize camera
    // Arbitrary height. Camera will go from -1 to 1
    let viewport_height: f32 = 2.0;
    // Width based on aspect ratio. With height of 2.0, we get 4. -2 to 2
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    // How far away the view plane is from the camera origin
    let focal_length: f32 = 1.0;

    // Camera origin
    let origin:Point3 = Point3::new(0.0, 0.0, 0.0);
    // Store the horizontal size of the view plane
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // Store the vertical size of the view plane
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // Get the lower left corner of the view plane. Used to convert the u&v values to the view plane
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Initialize ppm image file
    let mut output_file: File = File::create("output.ppm").expect("Unable to initiate file");
    output_file.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).expect("Unable to write to file");

    // Iterate over image
    for y in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {

            let u = x as f32 / (IMAGE_WIDTH) as f32;
            let v = 1.0 - (y as f32 / (IMAGE_HEIGHT) as f32);

            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let pixel_color = ray_color(r);
            write_color(&mut output_file, pixel_color);
        }
    }
    println!("Done!");
}