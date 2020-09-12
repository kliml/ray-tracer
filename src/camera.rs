use crate::ray::Ray;
use crate::rtweekend::*;
use crate::vec::*;
use rand::prelude::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f32,
        ascpect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = ascpect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray {
        let rd = random_in_unit_disc(rng) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
