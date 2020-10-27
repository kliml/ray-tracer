use crate::{aabb, hittable, material, ray, vec};
use std::rc::Rc;

pub struct Sphere {
    center: vec::Point3,
    radius: f32,
    material: Rc<dyn material::Material>,
}

impl Sphere {
    pub fn new(center: vec::Point3, radius: f32, material: Rc<dyn material::Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        ray: &ray::Ray,
        t_min: &mut f32,
        t_max: &mut f32,
        record: &mut hittable::HitRecord,
    ) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = vec::dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let mut temp = (-half_b - root) / a;
            if temp < *t_max && temp > *t_min {
                record.t = temp;
                record.p = ray.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_normale_face(ray, &outward_normal);
                record.material = self.material.clone();
                return true;
            }

            temp = (-half_b + root) / a;
            if temp < *t_max && temp > *t_min {
                record.t = temp;
                record.p = ray.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_normale_face(ray, &outward_normal);
                record.material = self.material.clone();
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, t0: f32, t1: f32, output_box: &mut aabb::Aabb) -> bool {
        *output_box = aabb::Aabb::new(
            self.center - vec::Vec3::new(self.radius, self.radius, self.radius),
            self.center + vec::Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
