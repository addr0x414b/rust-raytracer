use std::fs::File;
use std::io::Write;
use crate::Color;
use crate::rtweekend::clamp;

// Utility functions for colors

// Write the color to the file. Multiply by 255 to convert from 0 to 1 to 0 to 255
pub fn write_color(file: &mut File, pixel_color: Color, samples_per_pixel: u32) {
    let mut r: f32 = pixel_color.x();
    let mut g: f32 = pixel_color.y();
    let mut b: f32 = pixel_color.z();

    let scale: f32 = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    file.write(format!("{} {} {}\n", (255.0 * pixel_color.x()) as u16, (255.0 * pixel_color.y()) as u16, (255.0 * pixel_color.z()) as u16).as_bytes()).expect("Unable to write color pixel to file");
    //file.write(format!("{} {} {}\n", (255.0 * clamp(r, 0.0, 0.999)) as u16, (255.0 * clamp(g, 0.0, 0.999)) as u16, (255.0 * clamp(b, 0.0, 0.999)) as u16).as_bytes()).expect("Unable to write color pixel to file");
}