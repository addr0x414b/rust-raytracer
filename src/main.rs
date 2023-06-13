use std::fs::File;
use std::io::Write;

mod vec3;
use vec3::Point3;
use vec3::Color;
use vec3::Vec3;

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

            // Convert previous values from 0 to 1, to 0 to 255
            let ir: u16 = (255.0 * r) as u16;
            let ig: u16 = (255.0 * g) as u16;
            let ib: u16 = (255.0 * b) as u16;

            // Write to file
            output_file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write color pixel to file");
        }
    }
    println!("Done!");
}