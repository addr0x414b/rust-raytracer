use std::fs::File;
use std::io::Write;
fn main() {

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    let mut f = File::create("test.ppm").expect("Unable to create file");

    f.write_all((format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)).as_bytes()).expect("Unable to write to file");

    for j in (0..=IMAGE_HEIGHT-1).rev() {
        print!("Scanlines remaining: {}\n", j);
        for i in 0..=IMAGE_WIDTH-1 {
            let r = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let g = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write to file");
        }
    }
}
