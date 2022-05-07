use std::{fmt::Display, ops};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn unit_vector(&self) -> Self {
        self.clone() / self.length()
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

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });

impl_op_ex_commutative!(* |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b) });

impl_op_ex_commutative!(/ |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b) });

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

#[cfg(test)]
mod vec3_tests {
    use super::Vec3;

    #[test]
    fn new() {
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 2.0
            },
            Vec3::new(0.0, 1.0, 2.0)
        );
    }

    #[test]
    fn zero() {
        let zero = Vec3::zero();
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), zero);
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
        assert_eq!(Vec3::new(3.0, 6.0, 9.0,), vec1 + vec2);
    }

    #[test]
    fn sub_2_vec() {
        let vec1 = Vec3::new(2.0, 4.0, 6.0);
        let vec2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), vec1 - vec2);
    }

    #[test]
    fn mul_cst() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(2.0, 4.0, 6.0,), vec1 * 2.0);
    }

    #[test]
    fn div_cst() {
        let vec1 = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(Vec3::new(1.0, 2.0, 3.0,), vec1 / 2.0);
    }

    #[test]
    fn dot() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(20.0, vec1.dot(&vec2))
    }

    #[test]
    fn cross() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(Vec3::new(-1.0, 2.0, -1.0,), vec1.cross(&vec2))
    }

    #[test]
    fn unit_vector() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vec3::new(
                1.0 / (14 as f64).sqrt(),
                2.0 / (14 as f64).sqrt(),
                3.0 / (14 as f64).sqrt(),
            ),
            vec.unit_vector()
        )
    }
}
