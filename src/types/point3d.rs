use crate::types::matrix::Transformable;
use crate::types::{Axis, Double, Matrix, Vector3D};
use std::ops::{Add, Index, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3D {
    pub x: Double,
    pub y: Double,
    pub z: Double,
}

impl Point3D {
    pub fn zero() -> Point3D {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Sub for Point3D {
    type Output = Vector3D;
    fn sub(self, rhs: Point3D) -> Vector3D {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vector3D { x, y, z }
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Point3D { x, y, z }
    }
}

impl Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Point3D { x, y, z }
    }
}

impl Mul<Double> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: Double) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Point3D { x, y, z }
    }
}

impl Index<usize> for Point3D {
    type Output = Double;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Axis for Point3D {
    fn new(x: Double, y: Double, z: Double) -> Self {
        Point3D { x, y, z }
    }

    fn zero() -> Self {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn x(&self) -> Double {
        self.x
    }
    fn y(&self) -> Double {
        self.y
    }
    fn z(&self) -> Double {
        self.z
    }
}

// Matrix Multiplication
impl Mul<Matrix> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self.transform(rhs)
    }
}
