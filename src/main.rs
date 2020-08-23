use std::io;

mod color;
mod ray;
mod vec;

use ray::Ray;
use vec::{Color, Point3, Vec3};

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = vec::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image

    let ascpect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / ascpect_ratio) as i32;

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
            let pixel_color = ray_color(&ray);
            color::write_color(&mut handle, pixel_color);
        }
    }

    eprint!("\nDone.\n");
}
