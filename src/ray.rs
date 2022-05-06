use crate::vec3::{Point3, Vec3};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /**
    This function takes the parameter of the line's parametric equation and returns the
    corresponding point on the line.
    */
    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
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
    use crate::vec3::{Point3, Vec3};

    #[test]
    fn new() {
        assert_eq!(
            Ray {
                origin: Point3::new(1.0, 2.0, 3.0),
                direction: Vec3::new(2.0, 3.0, 4.0)
            },
            Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0))
        )
    }

    #[test]
    fn at() {
        assert_eq!(
            Point3::new(1.0, 1.0, 1.0),
            Ray::new(Point3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)).at(1.0)
        )
    }
}
