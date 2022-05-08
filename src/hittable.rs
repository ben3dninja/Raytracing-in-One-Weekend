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
    pub is_front_face: bool,
}

impl HitRecord {
    pub fn empty() -> Self {
        HitRecord {
            point: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            is_front_face: false,
        }
    }

    pub fn new(ray: &Ray, t: f64, outward_normal: &Vec3) -> Self {
        let is_front_face = ray.direction.dot(outward_normal) < 0.0;
        Self {
            t,
            point: ray.at(t),
            is_front_face,
            normal: if is_front_face {
                outward_normal.clone()
            } else {
                (-outward_normal).clone()
            }
        }
    }
}

#[allow(unused_variables)]
pub trait Hittable {
    /**
    This function returns a HitRecord if the specified ray actually hit the hittable, and None otherwise.
    */
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        None
    }
}
