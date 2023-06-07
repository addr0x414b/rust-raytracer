use std::fs::File;
use std::io::Write;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z:f64) -> Self {
        Self { x, y, z}
    }

    pub fn invert(&self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }

    pub fn add(&self, v: &Vec3) -> Vec3 {
        return Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z);
    }

    pub fn subtract(&self, v: &Vec3) -> Vec3 {
        return Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z);
    }

    pub fn multiply(&self, v: &Vec3) -> Vec3 {
        return Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z);
    }

    pub fn multiply_by(&self, n: f64) -> Vec3 {
        return Vec3::new(self.x * n, self.y * n, self.z *n);
    }

    pub fn divide_by(&self, n: f64) -> Vec3 {
        return Vec3::new(self.x * 1 as f64 / n, self.y * 1 as f64 / n, self.z * 1 as f64 / n);
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return (self.x*self.x) + (self.y*self.y) + (self.z*self.z);
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        return (self.x * v.x) + (self.y * v.y) + (self.z * v.z);
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        return Vec3::new(
            (self.y * v.z) - (self.z * v.y),
            (self.z * v.x) - (self.x * v.z),
            (self.x * v.y) - (self.y * v.x)
        );
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self.divide_by(self.length());
    }
}

fn write_color(f: &mut File, color: Vec3) {
    let ir: u32 = (255.999 * color.x) as u32;
    let ig: u32 = (255.999 * color.y) as u32;
    let ib: u32 = (255.999 * color.z) as u32;
    f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write to file");
}

fn main() {

    // Initialize image width and height
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Create the file to render to
    let mut f = File::create("test.ppm").expect("Unable to create file");

    // Add proper formatting to the file for ppm image
    f.write_all((format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)).as_bytes()).expect("Unable to write to file");

    // Loop through the image width and height
    for j in 0..=IMAGE_HEIGHT-1 {
        print!("Scanlines remaining: {}\n", IMAGE_HEIGHT - j); // Calculate remaining lines to render
        for i in 0..=IMAGE_WIDTH-1 {

            // Right now, just render a gradient
            let pixel_color: Vec3 = Vec3::new((i as f64) / (IMAGE_WIDTH - 1) as f64, (j as f64) / (IMAGE_HEIGHT - 1) as f64, 0.25);
            write_color(&mut f, pixel_color);
        }
    }
    print!("Done!\n");
}
