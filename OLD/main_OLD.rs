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

    pub fn clone(&self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z);
    }
}


pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return self.origin.add(&self.direction.multiply_by(t));
    }
}

fn write_color(f: &mut File, color: Vec3) {
    let ir: u32 = (255.999 * color.x) as u32;
    let ig: u32 = (255.999 * color.y) as u32;
    let ib: u32 = (255.999 * color.z) as u32;
    f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write to file");
}

fn unit_vector(v: Vec3) -> Vec3 {
    return v.divide_by(v.length());
}

fn ray_color(r: Ray) -> Vec3 {
    let unit_direction: Vec3 = unit_vector(r.direction);
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    return Vec3::new(1.0, 1.0, 1.0).multiply_by(1.0 - t).add(&Vec3::new(0.5, 0.7, 1.0).multiply_by(t));
}

fn main() {

    // Initialize image properties
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Initialize camera properties
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.subtract(&horizontal.divide_by(2.0)).subtract(&vertical.divide_by(2.0)).subtract(&Vec3::new(0.0, 0.0, focal_length));

    // Create the file to render to
    let mut f = File::create("test.ppm").expect("Unable to create file");

    // Add proper formatting to the file for ppm image
    f.write_all((format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)).as_bytes()).expect("Unable to write to file");

    // Loop through the image width and height
    for j in (0..=IMAGE_HEIGHT-1).rev() {
        print!("Scanlines remaining: {}\n", j); // Calculate remaining lines to render
        for i in 0..=IMAGE_WIDTH-1 {

            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let r = Ray::new(origin.clone(), lower_left_corner.add(&horizontal.multiply_by(u)).add(&vertical.multiply_by(v)).subtract(&origin));
            // Right now, just render a gradient
            let pixel_color = ray_color(r);
            write_color(&mut f, pixel_color);
        }
    }
    print!("Done!\n");
}
