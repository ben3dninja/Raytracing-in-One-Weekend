use crate::{vec3::{Point3, Vec3}, ray::Ray};

pub struct HitRecord {
    // The point where the ray hit the object
    pub point: Point3,
    // The object's normal vector on this point
    pub normal: Vec3,
    // The ray's t parameter, the point's antecendent
    pub t: f64,
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