use std::mem;

use crate::ray;
use crate::utility;
use crate::vec;

#[derive(Clone, Copy)]
pub struct Aabb {
    min: vec::Point3,
    max: vec::Point3,
}

impl Aabb {
    pub fn empty() -> Aabb {
        Aabb {
            min: vec::Point3::empty(),
            max: vec::Point3::empty(),
        }
    }

    pub fn new(min: vec::Point3, max: vec::Point3) -> Aabb {
        Aabb { min, max }
    }

    pub fn min(&self) -> vec::Point3 {
        self.min
    }

    pub fn max(&self) -> vec::Point3 {
        self.max
    }

    pub fn hit(&self, ray: &ray::Ray, tmin: &mut f32, tmax: &mut f32) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction()[i];
            let mut t0 = self.min()[i] - ray.origin()[i] * inv_d;
            let mut t1 = self.max()[i] - ray.origin()[i] * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            *tmin = if t0 > *tmin { t0 } else { *tmin };
            *tmin = if t1 < *tmax { t1 } else { *tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = vec::Point3::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );
    let big = vec::Point3::new(
        box0.max().x().max(box1.max().x()),
        box0.max().y().max(box1.max().y()),
        box0.max().z().max(box1.max().z()),
    );
    Aabb::new(small, big)
}
