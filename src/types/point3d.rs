use crate::types::{Double, Vector3D};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3D {
    x: Double,
    y: Double,
    z: Double,
}

impl Point3D {
    pub fn zero() -> Point3D {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn new(x: Double, y: Double, z: Double) -> Point3D {
        Point3D { x, y, z }
    }
}

impl Sub for Point3D {
    type Output = Vector3D;
    fn sub(self, rhs: Point3D) -> Vector3D {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Self::Output {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<Double> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: Double) -> Self::Output {
        Point3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
