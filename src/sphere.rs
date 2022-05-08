use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        // Solving the quadratic equation to determine the intersection points
        // between the ray and the sphere
        let oc = &ray.origin - &self.center;
        let a = (&ray.direction).length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        // The coefficients simplify themselves so the formula is shorter
        let delta = half_b * half_b - a * c;

        // If delta is negative, there are no roots, so the ray doesn't hit the sphere
        if delta < 0.0 {
            return None;
        }
        let sqrtd = delta.sqrt();

        // We first calculate the smaller root, closest to camera (if camera is at origin)
        let mut root = (-half_b - sqrtd) / a;

        // If the first root isn't in the bounds, calculate the second. If it isn't either,
        // return None
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (&ray.at(root) - &self.center) / self.radius;

        Some(HitRecord::new(&ray, root, &outward_normal))
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        hittable::Hittable,
        ray::Ray,
        vec3::{Point3, Vec3},
    };

    use super::Sphere;

    fn setup_sphere_and_ray() -> (Sphere, Ray) {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0);
        let ray = Ray::new(Point3::new(1.0, 0.0, 3.0), Vec3::new(0.0, 0.0, -2.0));
        (sphere, ray)
    }

    #[test]
    fn new() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0);
        assert_eq!(
            Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 2.0
            },
            sphere
        );
    }

    #[test]
    fn hit_true_when_hit() {
        let (sphere, ray) = setup_sphere_and_ray();
        assert!(sphere.hit(&ray, -5.0, 5.0).is_some())
    }

    #[test]
    fn hit_false_when_not_hit() {
        let (sphere, ray) = setup_sphere_and_ray();
        let ray = Ray {
            origin: Point3::new(0.0, 5.0, 0.0),
            ..ray
        };
        assert!(sphere.hit(&ray, -5.0, 5.0).is_none())
    }

    #[test]
    fn hit_correct_t_and_point() {
        let (sphere, ray) = setup_sphere_and_ray();
        let record = sphere.hit(&ray, -10.0, 10.0).unwrap();
        assert_eq!(Point3::new(1.0, 0.0, (3 as f64).sqrt()), record.point);
        assert_eq!((3.0 - (3 as f64).sqrt()) / 2.0, record.t);
    }

    #[test]
    fn hit_correct_face() {
        let (sphere, ray) = setup_sphere_and_ray();
        let ray = Ray {
            origin: Point3::new(1.0, 0.0, -3.0),
            ..ray
        };
        let record = sphere.hit(&ray, -1.0, 0.0).unwrap();
        assert!(!record.is_front_face);
    }
}
