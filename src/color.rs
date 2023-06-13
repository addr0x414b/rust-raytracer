use std::fs::File;
use std::io::Write;
use crate::Color;

// Utility functions for colors

// Write the color to the file. Multiply by 255 to convert from 0 to 1 to 0 to 255
pub fn write_color(file: &mut File, pixel_color: Color) {
    file.write(format!("{} {} {}\n", (255.0 * pixel_color.x()) as u16, (255.0 * pixel_color.y()) as u16, (255.0 * pixel_color.z()) as u16).as_bytes()).expect("Unable to write color pixel to file");
}