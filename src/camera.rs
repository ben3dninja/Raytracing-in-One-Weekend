use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(aspect_ratio: f64, focal_length: f64) -> Self {
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        let lower_left_corner =
            &origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn shoot_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            &self.lower_left_corner
                + u * &self.horizontal
                + v * &self.vertical
                - &self.origin
        )
    }
}
