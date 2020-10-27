use std::io;
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
//mod hittable_list;
mod aabb;
mod material;
mod ray;
mod sphere;
mod triangle;
mod utility;
mod vec;

use hittable::{HitRecord, Hittable};
use material::*;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use utility::*;
//use triangle::Triangle;
use vec::{Color, Point3, Vec3};

const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(ray: &mut Ray, world: &dyn Hittable, rng: &mut ThreadRng, depth: i32) -> Color {
    let mut record = HitRecord::empty();

    if depth <= 0 {
        return Color::empty();
    }

    if world.hit(ray, &mut 0.001, &mut INFINITY, &mut record) {
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

fn random_scene(rng: &mut ThreadRng) -> hittable::HittableList {
    let mut world = hittable::HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(rng);
            let center = Point3::new(
                a as f32 + 0.9 * random_double(rng),
                0.2,
                b as f32 + 0.9 * random_double(rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<dyn material::Material>;
                if choose_mat < 0.8 {
                    let albedo = Color::random(rng) * Color::random(rng);
                    material = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(rng, 0.5, 1.0);
                    let fuzz = random_double_range(rng, 0.0, 0.5);
                    material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    material = Rc::new(Dielectric::new(1.5));
                }
                world.add(Box::new(Sphere::new(center, 0.2, material.clone())));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));
    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));
    world
}

fn main() {
    let mut rng = rand::thread_rng();

    // Image

    let ascpect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f32 / ascpect_ratio) as i32;
    //let samples_per_pixel = 500;
    //let max_depth = 50;

    // World

    let mut world = random_scene(&mut rng);

    // let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // //let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    // //let material_center = Rc::new(Dielectric::new(1.5));
    // let material_left = Rc::new(Dielectric::new(1.5));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // let mut world = hittable_list::HittableList::new();

    // world.add(Box::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     material_center.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     -0.45,
    //     material_left.clone(),
    // )));

    // world.add(Box::new(Triangle::new(
    //     [
    //         Point3::new(0.0, 1.5, -1.0),
    //         Point3::new(0.5, -1.5, -1.0),
    //         Point3::new(-0.5, -1.5, -1.0),
    //     ],
    //     material_right.clone(),
    // )));

    // Camera

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = camera::Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ascpect_ratio,
        aperture,
        dist_to_focus,
    );

    // Renderer

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::empty();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_double(&mut rng)) / (image_width - 1) as f32;
                let v = (j as f32 + random_double(&mut rng)) / (image_height - 1) as f32;
                let mut ray = camera.get_ray(u, v, &mut rng);
                pixel_color = pixel_color + ray_color(&mut ray, &mut world, &mut rng, MAX_DEPTH);
            }
            color::write_color(&mut handle, pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
}
