use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    /**
    This function iterates through all the objects to find the closest one to the camera
    that the ray hits. It then returns the record of that hit.
    */
    pub fn trace_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(&ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{ray::Ray, sphere::Sphere};

    use super::HittableList;

    #[test]
    fn closest_record() {
        let sphere1 = Sphere::ni(-2, 0, 0, 0.5);
        let sphere2 = Sphere::ni(-4, 0, 0, 0.5);
        let ray = Ray::ni(0, 0, 0, -1, 0, 0);
        let mut world = HittableList::empty();
        world.add(Rc::new(sphere1));
        world.add(Rc::new(sphere2));
        assert_eq!(1.5, world.trace_ray(&ray, 0.0, 10.0).unwrap().t);
    }
}
