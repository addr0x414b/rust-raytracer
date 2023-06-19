use std::{fs::File, io::Write}; // Used to create/write to PPM file

mod vec3;
use crate::{vec3::{Vec3, Color, Point3}, ray::Ray};

mod ray;

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

fn ray_color(r: Ray) -> Color {
    return Color::new(1.0, 1.0, 1.0);
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
    println!("{}", IMAGE_HEIGHT);

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

            let u: f32 = (x / (IMAGE_WIDTH - 1)) as f32;
            let v: f32 = (y / (IMAGE_HEIGHT - 1)) as f32;

            let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);
            let color = ray_color(r);

            // Write our r,g,b values to every single pixel
            write_color(&mut output_file, color);
        }
    }
}