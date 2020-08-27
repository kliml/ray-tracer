use std::io;
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec;

use hittable::{HitRecord, Hittable};
use material::*;
use rand::prelude::*;
use ray::Ray;
use rtweekend::*;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};

fn ray_color(ray: &mut Ray, world: &dyn Hittable, rng: &mut ThreadRng, depth: i32) -> Color {
    let mut record = HitRecord::empty();

    if depth <= 0 {
        return Color::empty();
    }

    if world.hit(ray, 0.001, INFINITY, &mut record) {
        let mut scattered = Ray::empty();
        let mut attenuation = Color::empty();
        if record
            .material
            .scatter(ray, &record, &mut attenuation, &mut scattered, rng)
        {
            return attenuation * ray_color(&mut scattered, world, rng, depth - 1);
        }
        return Color::empty();
    }
    let unit_direction = vec::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image

    let ascpect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / ascpect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    // Camera

    let camera = camera::Camera::new();
    let mut rng = rand::thread_rng();

    // Renderer

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::empty();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_double(&mut rng)) / (image_width - 1) as f32;
                let v = (j as f32 + random_double(&mut rng)) / (image_height - 1) as f32;
                let mut ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&mut ray, &mut world, &mut rng, max_depth);
            }
            color::write_color(&mut handle, pixel_color, samples_per_pixel);
        }
    }
    eprint!("\nDone.\n");
}
