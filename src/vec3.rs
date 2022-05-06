use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[allow(dead_code)]
pub type Point3 = Vec3;
#[allow(dead_code)]
pub type Color = Vec3;

#[allow(dead_code)]
impl Color {
    pub fn write_color(self) {
        println!(
            "{} {} {}",
            (255.999 * self.x) as u32,
            (255.999 * self.y) as u32,
            (255.999 * self.z) as u32
        )
    }
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        super::Vec3::new($x, $y, $z)
    };
}

#[cfg(test)]
mod vec3_tests {
    use super::Vec3;

    #[test]
    fn new() {
        let vec = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 2.0
            },
            vec
        );
    }

    #[test]
    fn zero() {
        let zero = Vec3::zero();
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            zero
        );
    }

    #[test]
    fn length() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!((14 as f64).sqrt(), vec.length());
    }

    #[test]
    fn length_squared() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(14 as f64, vec.length_squared());
    }

    #[test]
    fn add_2_vec() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(
            Vec3 {
                x: 3.0,
                y: 6.0,
                z: 9.0
            },
            vec1 + vec2
        );
    }

    #[test]
    fn sub_2_vec() {
        let vec1 = Vec3::new(2.0, 4.0, 6.0);
        let vec2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            },
            vec1 - vec2
        );
    }

    #[test]
    fn mul_cst() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0,
            },
            vec1 * 2.0
        );
    }

    #[test]
    fn div_cst() {
        let vec1 = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            vec1 / 2.0
        );
    }

    #[test]
    fn dot() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(20.0, vec1.dot(vec2))
    }

    #[test]
    fn cross() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(
            Vec3 {
                x: -1.0,
                y: 2.0,
                z: -1.0,
            },
            vec1.cross(vec2)
        )
    }

    #[test]
    fn unit_vector() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vec3 {
                x: 1.0 / (14 as f64).sqrt(),
                y: 2.0 / (14 as f64).sqrt(),
                z: 3.0 / (14 as f64).sqrt(),
            },
            vec.unit_vector()
        )
    }
}
