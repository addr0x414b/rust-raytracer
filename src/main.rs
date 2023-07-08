use std::{fs::File, io::Write}; // Used to create/write to PPM file

mod vec3;
use vec3::{cross, unit_vector};

use crate::{vec3::{Vec3, Color, Point3, dot}, ray::Ray};

mod ray;
mod triangle;
use crate::triangle::Triangle;

mod mesh;
mod world;

/// Write a color in PPM format to a PPM file
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

/// Check if a triangle has been hit by the ray.
/// This code was provided by Wikipedia - 
/// 'Möller–Trumbore intersection algorithm' and translated
/// from C++ to  Rust by me.
/// # Arguments
/// * 't' - Triangle to check if ray has hit.
/// * 'r' - Ray from the camera.
fn hit_triangle(t: Triangle, r: Ray) -> f32{
    let edge1 = t[1] - t[0];
    let edge2 = t[2] - t[0];
    let h = cross(r.direction(), edge2);
    let a = dot(edge1, h);
    const EPSILON: f32 = 0.0000001;
    if a > -EPSILON && a < EPSILON {
        return -1.0;
    }

    let f = 1.0 / a;
    let s = r.origin() - t[0];
    let u = f * dot(s, h);
    if u < 0.0 || u > 1.0 {
        return -1.0;
    }

    let q = cross(s, edge1);
    let v = f * dot(r.direction(), q);
    if v < 0.0 || u + v > 1.0 {
        return -1.0;
    }

    let t = f * dot(edge2, q);
    if t > EPSILON {
        return t;
    }
    else {
        return -1.0;
    }
}

/// Calculate color based on the ray sent from the origin and returns the color.
/// Currently linearly interpolating from pure white to light blue.
/// # Arguments
/// 'r' - Ray type, contains the origin and its direction
fn ray_color(r: Ray) -> Color {
    // Test triangle
    let trig: Triangle = Triangle::new(
        Point3::new(-1.0, -0.5, -1.0),
        Point3::new(1.0, -0.5, -1.0),
        Point3::new(0.0, 0.5, -1.0),
        Vec3::new(0.0, 0.0, 1.0));
    let mut t = hit_triangle(trig, r);
    if t > 0.0 {
        //let n = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        let n = unit_vector(trig.normal());
        return Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0)*0.5;
    }
    let n = unit_vector(r.direction());
    t = (n.y() + 1.0) * 0.5;
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

    // Loop over every single pixel in our image
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            // Color each pixel light blue
            //let color: Color = Color::new(0.56, 0.64, 0.96);

            let u: f32 = x as f32 / (IMAGE_WIDTH - 1) as f32;
            let v: f32 = 1.0 - (y as f32 / (IMAGE_HEIGHT - 1) as f32);

            let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);
            let color = ray_color(r);

            // Write our r,g,b values to every single pixel
            write_color(&mut output_file, color);
        }
    }
}