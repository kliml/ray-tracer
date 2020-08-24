use std::io;

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec;

use hittable::{HitRecord, Hittable};
use ray::Ray;
use rtweekend::*;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};

// fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
//     let oc = ray.origin() - center;
//     let a = ray.direction().length_squared();
//     let half_b = vec::dot(&oc, &ray.direction());
//     let c = oc.length_squared() - radius * radius;
//     let discriminant = half_b * half_b - a * c;
//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-half_b - discriminant.sqrt()) / a
//     }
// }

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    // let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    let mut record = HitRecord::empty();
    if world.hit(ray, 0.0, INFINITY, &mut record) {
        return (record.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    // if t > 0.0 {
    //     let n = vec::unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
    //     return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    // }
    let unit_direction = vec::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image

    let ascpect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / ascpect_ratio) as i32;

    // World

    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = ascpect_ratio * viewport_height;
    let focal_lenght = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_lenght);

    // Renderer

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&ray, &mut world);
            color::write_color(&mut handle, pixel_color);
        }
    }

    eprint!("\nDone.\n");
}
