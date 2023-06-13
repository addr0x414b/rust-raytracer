use std::fs::File;
use std::io::Write;

mod vec3;
mod color;

use crate::vec3::Color;
use crate::color::write_color;


fn main() {

    // Initialize image properties
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;

    // Initialize ppm image file
    let mut output_file: File = File::create("output.ppm").expect("Unable to initiate file");
    output_file.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).expect("Unable to write to file");

    // Iterate over image
    for y in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            // Calculate gradient. Result is in between 0 and 1
            // Closer to the right we get, the more red. Closer to the top, more green
            let r: f32 = x as f32 / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = 1.0 - (y as f32 / (IMAGE_HEIGHT - 1) as f32);
            let b: f32 = 0.25;

            let pixel_color: Color = Color::new(r, g, b);
            write_color(&mut output_file, pixel_color);
        }
    }
    println!("Done!");
}