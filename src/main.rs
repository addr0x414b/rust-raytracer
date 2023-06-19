use std::{fs::File, io::Write}; // Used to create/write to PPM file
mod vec3;
use crate::vec3::Color;

/// Write a color in PPM format to a PPM file
/// # Arguments
/// * 'file' - Output PPM file, should already be initialized.
/// * 'color' - Color struct which contains x,y,z (rgb). 0 <= R,G,B <= 255.
fn write_color(file: &mut File, color: Color) {
    if color.x() > 255.0 || color.y() > 255.0 || color.z() > 255.0 {
        panic!("write_color: R,G,B values are larger than 255");
    }
    file.write(format!("{} {} {}\n", color.x() as u16, color.y() as u16, color.z() as u16).as_bytes())
        .expect("Unable to write to file");
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
    for _y in 0..IMAGE_HEIGHT {
        for _x in 0..IMAGE_WIDTH {
            // Color each pixel light blue
            let color: Color = Color::new(163.9, 205.0, 244.0);

            // Write our r,g,b values to every single pixel
            write_color(&mut output_file, color);
        }
    }
}