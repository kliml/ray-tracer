use crate::{aabb, hittable, material, ray, vec, vec::*};
use std::rc::Rc;

pub struct Triangle {
    vertices: [Point3; 3],
    material: Rc<dyn material::Material>,
}

impl Triangle {
    pub fn new(vertices: [Point3; 3], material: Rc<dyn material::Material>) -> Triangle {
        Triangle { vertices, material }
    }
}

// Möller-Trumbore algorithm
impl hittable::Hittable for Triangle {
    fn hit(
        &self,
        ray: &ray::Ray,
        t_min: &mut f32,
        t_max: &mut f32,
        record: &mut hittable::HitRecord,
    ) -> bool {
        let e1 = self.vertices[0] - self.vertices[1];
        let e2 = self.vertices[0] - self.vertices[2];

        let normal = vec::cross(&e1, &e2);
        // eprintln!("{}", normal);

        // Calculating normal to surface
        let pvec = vec::cross(&ray.direction(), &e2);
        let det = vec::dot(&e1, &pvec);

        // If parallel to surface
        if det < f32::EPSILON && det > -f32::EPSILON {
            return false;
        }

        let inv_det = det.recip();
        let tvec = ray.origin() - self.vertices[0];
        let u = vec::dot(&tvec, &pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let qvec = vec::cross(&tvec, &e1);
        let v = vec::dot(&ray.direction(), &qvec);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = vec::dot(&e2, &qvec) * inv_det;
        if t < *t_max && t > *t_min {
            record.t = vec::dot(&e2, &qvec) * inv_det;
            record.p = ray.at(record.t);
            // outward normal
            // record.normal = normal;
            // normale face
            record.set_normale_face(ray, &normal);
            record.material = self.material.clone();
            return true;
        }
        false
    }

    fn bounding_box(&self, t0: f32, t1: f32, output_box: &mut aabb::Aabb) -> bool {
        unimplemented!();
    }
}
