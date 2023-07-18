// Things I didn't write
use std::{fs::File, io::Write};
use rand::Rng;

// Things I wrote
mod vec3;
mod triangle;
mod mesh;
mod ray;
mod world;
mod hit;
mod material;
use vec3::{barycentric, unit_vector};
use crate::{vec3::Vec3, ray::Ray, world::World, triangle::Triangle, mesh::{Mesh, load_mesh}, material::{Material, MaterialEnum, Diffuse, Metal}};

#[derive(Copy, Clone, Debug)]
enum DrawingMode {
    Colors,
    Normals,
    Samples
}

fn ray_color(r: Ray, w: World, depth: u32, mode: DrawingMode) -> Vec3 {
    let hit = w.hit(r);
    match mode {
        DrawingMode::Colors => {
            if hit.t > 0.0 {
                return hit.material.get_albedo();
            }
        },
        DrawingMode::Normals => {
            if hit.t > 0.0 {
                let n: Vec3;
                if hit.triangle.smooth {
                    let bary = barycentric(hit.clone());
                    n = unit_vector(hit.triangle.normals[0] * bary.x + hit.triangle.normals[1] * bary.y + hit.triangle.normals[2] * bary.z);
                } else {
                    n = hit.triangle.normal;
                }
                return Vec3::new(n.x+1.0, n.y+1.0, n.z+1.0) * 0.5;
            }
        },
        DrawingMode::Samples => {
            if depth <= 0 {
                return Vec3::new(0.0, 0.0, 0.0);
            }
            if hit.t > 0.0 {
                let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
                let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
                if hit.material.scatter(r, hit.clone(), &mut attenuation, &mut scattered) {
                    return attenuation * ray_color(scattered, w.clone(), depth-1, mode.clone());
                }
            }
        }
    }
    // This code generates the blueish gradient background
    let n = r.direction;
    let t = (n.y + 1.0) * 0.5;
    // Typical interpolation
    return (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + Vec3::new(0.5, 0.7, 1.0)*t;
    //return Vec3::new(1.0, 1.0, 1.0);
}

fn write_color(file: &mut File, color: Vec3, samples: u32, mode: DrawingMode) {
    let r: u32;
    let g: u32;
    let b: u32;
    match mode {
        DrawingMode::Colors | DrawingMode::Normals => {
            r = (color.x * 255.0) as u32;
            g = (color.y * 255.0) as u32;
            b = (color.z * 255.0) as u32;
        },
        DrawingMode::Samples => {
            r = ((color.x * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;
            g = ((color.y * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;
            b = ((color.z * (1.0 / samples as f64)).sqrt().clamp(0.0, 0.999) * 255.0) as u32;

        }
    }
    if r > 255 || g > 255 || b > 255 {
        panic!("Color value out of range");
    }

    file.write(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("Unable to write to output file");
}

fn main() {

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const DRAWING_MODE: DrawingMode = DrawingMode::Colors;
    const SAMPLES : u32 = 100;
    const MAX_DEPTH: u32 = 5;

    let mut output_file = File::create("output.ppm")
        .expect("Failed to create PPM file");

    output_file.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .expect("Failed to write to PPM file");

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 5.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);

    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);

    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Teapot final scene
    /*let mut floor = load_mesh("models/plane.obj", false);
    floor.scale(4.0);
    floor.rotate(Vec3::new(0.0, 0.0, 0.0));
    floor.translate(Vec3::new(0.0, -1.4, -10.0));
    floor.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.6, 0.4, 0.7)));

    let mut teapot = load_mesh("models/teapot_smooth.obj", true);
    teapot.scale(0.15);
    teapot.rotate(Vec3::new(0.0, -20.0, 0.0));
    teapot.translate(Vec3::new(0.0, -1.5, -10.0));
    teapot.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.5, 0.9, 0.5), 0.3));

    let mut cube = load_mesh("models/diamond.obj", false);
    cube.scale(0.5);
    cube.rotate(Vec3::new(0.0, 30.0, 0.0));
    cube.translate(Vec3::new(2.5, -0.9, -10.0));
    cube.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.6, 0.2, 0.2), 0.0));

    let mut cube2 = load_mesh("models/cube.obj", false);
    cube2.scale(0.25);
    cube2.rotate(Vec3::new(0.0, 90.0, 0.0));
    cube2.translate(Vec3::new(2.5, -0.2, -10.0));
    cube2.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.8, 0.8, 0.4)));

    let mut sphere = load_mesh("models/sphere_smooth.obj", true);
    sphere.scale(0.5);
    sphere.rotate(Vec3::new(0.0, 45.0, 0.0));
    sphere.translate(Vec3::new(-3.0, -0.9, -10.0));
    sphere.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.7, 1.0, 0.2), 0.1));

    let mut sphere2 = load_mesh("models/sphere_smooth.obj", true);
    sphere2.scale(0.5);
    sphere2.rotate(Vec3::new(0.0, 45.0, 0.0));
    sphere2.translate(Vec3::new(-2.5, -0.9, -12.0));
    sphere2.material = MaterialEnum::Metal(Metal::new(Vec3::new(1.0, 0.8, 0.2), 0.3));

    let mut mirror = load_mesh("models/cube.obj", false);
    mirror.scale(2.0);
    mirror.rotate(Vec3::new(15.0, 0.0, 0.0));
    mirror.translate(Vec3::new(0.0, 1.0, -17.0));
    mirror.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.7, 0.7, 0.7), 0.0));

    let mut mirror2 = load_mesh("models/cube.obj", false);
    mirror2.scale(2.0);
    mirror2.rotate(Vec3::new(15.0, 20.0, 0.0));
    mirror2.translate(Vec3::new(-5.0, 1.0, -17.0));
    mirror2.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.7, 0.7, 0.7), 0.0));

    let mut world = World::new();
    world.add(floor);
    world.add(teapot);
    world.add(cube);
    world.add(cube2);
    world.add(sphere);
    world.add(sphere2);*/



    // Cube and sphere
    /*let mut floor = load_mesh("models/plane.obj", false);
    floor.scale(4.0);
    floor.rotate(Vec3::new(0.0, 0.0, 0.0));
    floor.translate(Vec3::new(0.0, -1.4, -10.0));
    floor.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.6, 0.4, 0.7)));

    let mut sphere = load_mesh("models/sphere_smooth.obj", true);
    sphere.translate(Vec3::new(0.0, -0.4, -8.0));
    sphere.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.89, 0.9, 0.3), 0.0));

    let mut cube = load_mesh("models/cube.obj", false);
    cube.translate(Vec3::new(-2.5, -0.4, -8.25));
    cube.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.3, 0.0, 0.0)));

    let mut cube2 = load_mesh("models/cube.obj", false);
    cube2.translate(Vec3::new(2.5, -0.4, -8.25));
    cube2.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.0, 0.0, 0.3)));

    let mut world = World::new();
    world.add(floor);
    world.add(sphere);
    world.add(cube);
    world.add(cube2);*/

    let mut floor = load_mesh("models/plane.obj", false);
    floor.scale(4.0);
    floor.rotate(Vec3::new(0.0, 0.0, 0.0));
    floor.translate(Vec3::new(0.0, -1.4, -10.0));
    floor.material = MaterialEnum::Metal(Metal::new(Vec3::new(0.89, 0.4, 0.4), 0.0));

    let mut chess = load_mesh("models/chess.obj", false);
    chess.scale(0.25);
    chess.translate(Vec3::new(0.0, -1.4, -12.0));
    chess.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.8, 0.8, 0.4)));

    let mut teapot = load_mesh("models/teapot_smooth.obj", true);
    teapot.scale(0.1);
    teapot.translate(Vec3::new(-2.5, -1.4, -12.0));
    teapot.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.5, 0.9, 0.9)));

    let mut cube = load_mesh("models/cube.obj", false);
    cube.scale(1.0);
    cube.rotate(Vec3::new(0.0, 10.0, 0.0));
    cube.translate(Vec3::new(2.5, -0.4, -12.0));
    cube.material = MaterialEnum::Diffuse(Diffuse::new(Vec3::new(0.8, 0.5, 1.0)));

    let mut world = World::new();
    world.add(floor);
    world.add(chess);
    world.add(teapot);
    world.add(cube);



    for y in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", y+1);
        for x in 0..IMAGE_WIDTH {
            match DRAWING_MODE {
                DrawingMode::Colors | DrawingMode::Normals => {
                    let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
                    let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;

                    let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);
                    let color = ray_color(r, world.clone(), MAX_DEPTH, DRAWING_MODE);

                    write_color(&mut output_file, color, 1, DRAWING_MODE);
                },
                DrawingMode::Samples => {
                    let mut color = Vec3::new(0.0, 0.0, 0.0);

                    for _ in 0..SAMPLES {
                        let mut rng = rand::thread_rng(); // Need to randomly pick samples around our pixel
                        let u: f64 = ((x) as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                        let v: f64 = ((y as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64);

                        let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v) - origin);
                        color = color + ray_color(r, world.clone(), MAX_DEPTH, DRAWING_MODE);
                    }
                    write_color(&mut output_file, color, SAMPLES, DRAWING_MODE);
                }
            }
        }
    }
}