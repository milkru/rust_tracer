mod vec;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use rayon::prelude::IntoParallelIterator;
use vec::Vec3;
use ray::Ray;
use hittable::{Hittable, Scene};
use sphere::Sphere;
use camera::Camera;
use rand::Rng;
use material::{Lambertian, Metal, Dielectric};
use std::sync::Arc;
use rayon::iter::ParallelIterator;

fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    
    if let Some(rec) = scene.hit(ray, 0.001, f32::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(ray, &rec) {
            let color = ray_color(&scattered, scene, depth - 1);
            Vec3::new(color.x * attenuation.x, color.y * attenuation.y, color.z * attenuation.z)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let dir_norm = ray.dir.norm();
        let t = 0.5 * (dir_norm.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(251.0 / 256.0, 193.0 / 256.0, 155.0 / 256.0)
    }
}

fn setup_random_scene() -> Scene {
    let mut scene = Scene::new();
    let mut rng = rand::thread_rng();

    let mat_ground = Arc::new(Lambertian::new(Vec3::new(139.0 / 256.0, 175.0 / 256.0, 197.0 / 256.0)));
    scene.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground)));
    
    for i in -11..=11 {
        for j in -11..=11 {
            let rand_mat_determinant = rng.gen::<f32>();
            let sphere_center = Vec3::new(
                (i as f32) + rng.gen_range(0.0..0.9),
                0.2,
                (j as f32) + rng.gen_range(0.0..0.9));

            if rand_mat_determinant < 0.8 {
                // Diffuse
                let first_random_col = Vec3::get_random(0.0..1.0);
                let second_random_col = Vec3::get_random(0.0..1.0);
                let albedo = Vec3::new(
                    first_random_col.x * second_random_col.x,
                    first_random_col.y * second_random_col.y,
                    first_random_col.z * second_random_col.z);

                let mat = Arc::new(Lambertian::new(albedo));
                scene.push(Box::new(Sphere::new(sphere_center, 0.2, mat)));
            } else if rand_mat_determinant < 0.95 {
                // Metal
                let albedo = Vec3::get_random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);

                let mat = Arc::new(Metal::new(albedo, fuzz));
                scene.push(Box::new(Sphere::new(sphere_center, 0.2, mat)));
            } else {
                // Glass
                let mat = Arc::new(Dielectric::new(1.5));
                scene.push(Box::new(Sphere::new(sphere_center, 0.2, mat)));
            }
        }
    }

    let mat = Arc::new(Dielectric::new(1.5));
    scene.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat)));
    
    let mat = Arc::new(Lambertian::new(Vec3::new(0.2, 0.4, 0.8)));
    scene.push(Box::new(Sphere::new(Vec3::new(1.5, 1.0, 2.0), 1.0, mat)));

    let mat = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    scene.push(Box::new(Sphere::new(Vec3::new(3.0, 1.0, 0.0), 1.0, mat)));

    scene
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 1280;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIX: u32 = 512;
    const MAX_RAY_DEPTH: u32 = 5;

    // Scene
    let scene = setup_random_scene();

    // Camera
    let lookfrom = Vec3::new(-1.0, 2.0, -13.0);
    let lookat = Vec3::new(1.5, 0.5, 0.0);

    let camera = Camera::new(lookfrom,
                        lookat,
                        Vec3::new(0.0, 1.0, 0.0),
                        20.0,
                        ASPECT_RATIO,
                        0.1,
                        10.0);
    
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    
    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Remaining scanlines: {}", i + 1);

        let scanline: Vec<Vec3> = (0..IMAGE_WIDTH).into_par_iter().map(|j| {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIX {
                let mut rng = rand::thread_rng();
                
                let u = ((j as f32) + rng.gen::<f32>()) / ((IMAGE_WIDTH - 1) as f32);
                let v = ((i as f32) + rng.gen::<f32>()) / ((IMAGE_HEIGHT - 1) as f32);
                
                let cam_ray = camera.get_ray(u, v);
                col = col + ray_color(&cam_ray, &scene, MAX_RAY_DEPTH);
            }
            col
        }).collect();

        for col in scanline {
            println!("{} {} {}",
            (256.0 * (col.x / (SAMPLES_PER_PIX as f32)).sqrt().clamp(0.0, 0.999)) as u32,
            (256.0 * (col.y / (SAMPLES_PER_PIX as f32)).sqrt().clamp(0.0, 0.999)) as u32,
            (256.0 * (col.z / (SAMPLES_PER_PIX as f32)).sqrt().clamp(0.0, 0.999)) as u32);
        }
    }
}
