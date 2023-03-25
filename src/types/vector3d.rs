use crate::types::{Axis, Double, Matrix};
use crate::types::matrix::Transformable;
use std::ops::{Add, BitXor, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3D {
    pub x: Double,
    pub y: Double,
    pub z: Double,
}

impl Axis for Vector3D {
    fn new(x: Double, y: Double, z: Double) -> Self {
        Vector3D { x, y, z }
    }

    fn zero() -> Vector3D {
        Vector3D {
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

impl Vector3D {
    pub fn normal(x: Double, y: Double, z: Double) -> Vector3D {
        let mut v = Vector3D { x, y, z };
        v.normalize();
        v
    }

    pub fn normalize(&mut self) -> Vector3D {
        let length = self.magnitude();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        *self
    }

    pub fn magnitude(&self) -> Double {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn component_mul(&self, v: Vector3D) -> Vector3D {
        let x = self.x * v.x;
        let y = self.y * v.y;
        let z = self.z * v.z;
        Vector3D { x, y, z }
    }

    pub const UP: Vector3D = Vector3D {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
}

// Scalar Multiplication
impl Mul<Double> for Vector3D {
    type Output = Vector3D;
    fn mul(self, r: Double) -> Vector3D {
        let x = r * self.x;
        let y = r * self.y;
        let z = r * self.z;
        Vector3D { x, y, z }
    }
}

impl Mul<Vector3D> for Double {
    type Output = Vector3D;
    fn mul(self, r: Vector3D) -> Vector3D {
        r * self
    }
}

impl Mul for Vector3D {
    type Output = Double;
    fn mul(self, r: Vector3D) -> Double {
        self.x * r.x + self.y * r.y + self.z * r.z
    }
}

// Cross Product
impl BitXor<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn bitxor(self, v: Vector3D) -> Vector3D {
        let x = self.y * v.z - self.z * v.y;
        let y = self.z * v.x - self.z * v.z;
        let z = self.x * v.y - self.y * v.x;
        Vector3D { x, y, z }
    }
}

// Addition
impl Add for Vector3D {
    type Output = Vector3D;
    fn add(self, r: Vector3D) -> Vector3D {
        let x = self.x + r.x;
        let y = self.y + r.y;
        let z = self.z + r.z;
        Vector3D { x, y, z }
    }
}

// Subtraction
impl Sub for Vector3D {
    type Output = Vector3D;
    fn sub(self, r: Vector3D) -> Vector3D {
        let x = self.x - r.x;
        let y = self.y - r.y;
        let z = self.z - r.z;
        Vector3D { x, y, z }
    }
}

// Negation
impl Neg for Vector3D {
    type Output = Vector3D;
    fn neg(self) -> Vector3D {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Vector3D { x, y, z }
    }
}

// Division
impl Div<Double> for Vector3D {
    type Output = Vector3D;
    fn div(self, r: Double) -> Vector3D {
        let x = self.x / r;
        let y = self.y / r;
        let z = self.z / r;
        Vector3D { x, y, z }
    }
}

// Not-Equal to a float
impl PartialEq<Double> for Vector3D {
    fn eq(&self, v: &Double) -> bool {
        self.x == *v || self.y == *v || self.z == *v
    }
}

// Index-Based Access
impl Index<usize> for Vector3D {
    type Output = Double;
    fn index(&self, i: usize) -> &Double {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, i: usize) -> &mut Double {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}


impl Mul<Matrix> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self.transform(rhs)
    }
}