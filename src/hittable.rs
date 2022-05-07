use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    // The point where the ray hit the object
    pub point: Point3,
    // The object's normal vector on this point
    pub normal: Vec3,
    // The ray's t parameter, the point's antecendent
    pub t: f64,
    // The normal's orientation
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> Self {
        HitRecord {
            point: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            (-outward_normal).clone()
        }
    }
}

#[allow(unused_variables)]
pub trait Hittable {
    /**
    This function returns true if the ray hits the object with a t in the desired range, and false otherwise.
    In addition, it modifies the HitRecord accordingly.
    */
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        false
    }
}
