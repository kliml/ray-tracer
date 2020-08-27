use crate::{hittable, ray, vec};
use rand::prelude::*;

pub trait Material {
    fn scatter(
        &self,
        ray: &mut ray::Ray,
        record: &hittable::HitRecord,
        attenuation: &mut vec::Color,
        scattered: &mut ray::Ray,
        rng: &mut ThreadRng,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: vec::Color,
}

impl Lambertian {
    pub fn new(albedo: vec::Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &mut ray::Ray,
        record: &hittable::HitRecord,
        attenuation: &mut vec::Color,
        scattered: &mut ray::Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let scatter_direction = record.normal + vec::Vec3::random_unit_vector(rng);
        *scattered = ray::Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: vec::Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: vec::Color, fuzz: f32) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &mut ray::Ray,
        record: &hittable::HitRecord,
        attenuation: &mut vec::Color,
        scattered: &mut ray::Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let reflected = vec::reflect(&ray.direction(), &record.normal);
        *scattered = ray::Ray::new(
            record.p,
            reflected + vec::Vec3::random_in_unit_sphere(rng) * self.fuzz,
        );
        *attenuation = self.albedo;
        vec::dot(&scattered.direction(), &record.normal) > 0.0
    }
}

pub struct Dielectric {
    pub reflection_index: f32,
}

impl Dielectric {
    pub fn new(reflection_index: f32) -> Dielectric {
        Dielectric { reflection_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &mut ray::Ray,
        record: &hittable::HitRecord,
        attenuation: &mut vec::Color,
        scattered: &mut ray::Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        *attenuation = vec::Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if record.front_face {
            1.0 / self.reflection_index
        } else {
            self.reflection_index
        };
        let unit_direction = vec::unit_vector(ray.direction());
        let refracted = vec::refract(&unit_direction, &record.normal, etai_over_etat);
        *scattered = ray::Ray::new(record.p, refracted);
        true
    }
}
