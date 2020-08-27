use crate::{material, ray, vec};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: vec::Point3,
    pub normal: vec::Vec3,
    pub material: Rc<dyn material::Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            p: vec::Point3::empty(),
            normal: vec::Vec3::empty(),
            material: Rc::new(material::Lambertian {
                albedo: vec::Color::new(0.0, 0.0, 0.0),
            }),
            t: f32::MIN,
            front_face: true,
        }
    }

    pub fn set_normale_face(&mut self, ray: &ray::Ray, outward_normal: &vec::Vec3) {
        self.front_face = vec::dot(&ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}
