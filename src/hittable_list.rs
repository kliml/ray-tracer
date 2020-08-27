use crate::{hittable, ray, vec};

pub struct HittableList {
    objects: Vec<Box<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn hittable::Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl hittable::Hittable for HittableList {
    fn hit(
        &self,
        ray: &ray::Ray,
        t_min: f32,
        t_max: f32,
        record: &mut hittable::HitRecord,
    ) -> bool {
        let mut temp_record = hittable::HitRecord::empty();
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}
