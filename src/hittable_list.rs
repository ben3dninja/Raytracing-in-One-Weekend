use std::rc::Rc;

use crate::{hittable::{Hittable, HitRecord}, ray::Ray};

struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

#[allow(dead_code)]
impl HittableList {
    fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let temp_rec = object.hit(&ray, t_min, closest_so_far);
            if temp_rec.is_some() {
                closest_so_far = temp_rec.as_ref().unwrap().t;
                rec = temp_rec;
            }
        }
        rec
    }
}