use crate::{aabb, material, ray, vec};
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
    fn hit(&self, ray: &ray::Ray, t_min: &mut f32, t_max: &mut f32, record: &mut HitRecord)
        -> bool;
    fn bounding_box(&self, t0: f32, t1: f32, output_box: &mut aabb::Aabb) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &ray::Ray,
        t_min: &mut f32,
        t_max: &mut f32,
        record: &mut HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                *closest = temp_record.t;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32, output_box: &mut aabb::Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        };

        let mut temp_box: aabb::Aabb = aabb::Aabb::empty();
        let mut first_box = true;

        for o in self.objects.iter() {
            if !o.bounding_box(t0, t1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                aabb::surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }

        true
    }
}

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    _box: aabb::Aabb,
}

impl BvhNode {}

impl Hittable for BvhNode {
    fn hit(
        &self,
        ray: &ray::Ray,
        t_min: &mut f32,
        t_max: &mut f32,
        record: &mut HitRecord,
    ) -> bool {
        if !self._box.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, record);
        let hit_right = self.left.hit(ray, t_min, t_max, record);

        hit_left || hit_right
    }

    fn bounding_box(&self, t0: f32, t1: f32, output_box: &mut aabb::Aabb) -> bool {
        unimplemented!();
    }
}
