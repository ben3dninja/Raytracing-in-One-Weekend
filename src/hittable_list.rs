use std::rc::Rc;

use crate::{hittable::{Hittable, HitRecord}, ray::Ray};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

#[allow(dead_code)]
impl HittableList {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    /**
     This function iterates through all the objects to find the closest one to the camera
     that the ray hits. It then returns the record of that hit.
     */
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec ) = object.hit(&ray, t_min, closest_so_far) {
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

    use crate::{sphere::Sphere, vec3::{Point3, Vec3}, ray::Ray};

    use super::HittableList;

    #[test]
    fn closest_record() {
        let sphere1 = Sphere::new(Point3::new(-2.0, 0.0, 0.0), 0.5);
        let sphere2 = Sphere::new(Point3::new(-4.0, 0.0, 0.0), 0.5);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
        let mut world = HittableList::empty();
        world.add(Rc::new(sphere1));
        world.add(Rc::new(sphere2));
        assert_eq!(1.5, world.hit(&ray, 0.0, 10.0).unwrap().t);
    }
}