use crate::vec3::{Point3, Vec3};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn n(x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) -> Self {
        Ray {
            origin: Point3::new(x, y, z),
            direction: Vec3::new(dx, dy, dz),
        }
    }

    pub fn ni(x: i32, y: i32, z: i32, dx: i32, dy: i32, dz: i32) -> Self {
        Ray::n(
            x as f64, y as f64, z as f64, dx as f64, dy as f64, dz as f64,
        )
    }

    /**
    This function takes the t parameter of the line's parametric equation and returns the
    corresponding point on the line.
    */
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.direction
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::vec3::Point3;

    #[test]
    fn at() {
        assert_eq!(Point3::ni(1, 1, 1), Ray::ni(1, 1, 0, 0, 0, 1).at(1.0))
    }
}
