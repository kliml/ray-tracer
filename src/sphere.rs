use crate::{hittable, ray, vec};

pub struct Sphere {
    center: vec::Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: vec::Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        ray: &ray::Ray,
        t_min: f32,
        t_max: f32,
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
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_normale_face(ray, &outward_normal);
                return true;
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_normale_face(ray, &outward_normal);
                return true;
            }
        }
        false
    }
}
