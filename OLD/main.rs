use std::f32::INFINITY;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittableList;
mod rtweekend;
mod camera;

use camera::Camera;
use hittable::HitRecord;
use hittable::Hittable;
use vec3::dot;

use crate::hittableList::HittableList;
use crate::rtweekend::random_double;
use crate::sphere::Sphere;
use crate::vec3::unit_vector;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::color::write_color;


// Check if we hit a sphere based on its center point and radius
fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    // Equation to calculate if a ray hit a sphere
    let oc: Vec3 = r.origin() - center;
    let a: f32 = r.direction().length_squared();
    let half_b: f32 = dot(oc, r.direction());
    let c: f32 = oc.length_squared() - radius*radius;
    let discriminant: f32 = half_b*half_b - a*c;
    
    // If we're less than 0, we didn't hit the sphere
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

// Either draw the background or if we hit a sphere, draw the sphere
fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = unit_vector(r.direction());
    let t = (unit_direction.y() + 1.0) * 0.5;
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {

    // Initialize image properties
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam: Camera = Camera::new();
    // Initialize camera
    // Arbitrary height. Camera will go from -1 to 1
    let viewport_height: f32 = 2.0;
    // Width based on aspect ratio. With height of 2.0, we get 4. -2 to 2
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    // How far away the view plane is from the camera origin
    let focal_length: f32 = 1.0;

    // Camera origin
    let origin:Point3 = Point3::new(0.0, 0.0, 0.0);
    // Store the horizontal size of the view plane
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // Store the vertical size of the view plane
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // Get the lower left corner of the view plane. Used to convert the u&v values to the view plane
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Initialize ppm image file
    let mut output_file: File = File::create("output.ppm").expect("Unable to initiate file");
    output_file.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).expect("Unable to write to file");

    // Iterate over image
    for y in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (x as f32 + random_double()) / (IMAGE_WIDTH) as f32;
                let v: f32 =1.0 - ((y as f32 + random_double()) / (IMAGE_HEIGHT) as f32);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(&mut output_file, pixel_color, SAMPLES_PER_PIXEL);

            /*// Internal position of our image
            let u = x as f32 / (IMAGE_WIDTH) as f32;
            let v = 1.0 - (y as f32 / (IMAGE_HEIGHT) as f32);

            // Create a ray. We store the origin, and then we also calculate the direction we are pointing at based on the pixel value of our image
            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);

            // Grab the color value of the ray and draw it
            let pixel_color = ray_color(&r, &world);
            write_color(&mut output_file, pixel_color, SAMPLES_PER_PIXEL);*/
        }
    }
    println!("Done!");
}