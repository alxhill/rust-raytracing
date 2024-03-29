use crate::types::Double;
use std::ops::{Add, BitXor, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3D {
    pub x: Double,
    pub y: Double,
    pub z: Double,
}

impl Vector3D {
    pub const fn new(x: Double, y: Double, z: Double) -> Vector3D {
        Vector3D { x, y, z }
    }

    pub fn zero() -> Vector3D {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn normal(x: Double, y: Double, z: Double) -> Vector3D {
        let mut v = Vector3D::new(x, y, z);
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
        Vector3D::new(self.x * v.x, self.y * v.y, self.z * v.z)
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
        Vector3D::new(r * self.x, r * self.y, r * self.z)
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
        Vector3D::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.z * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

// Addition
impl Add for Vector3D {
    type Output = Vector3D;
    fn add(self, r: Vector3D) -> Vector3D {
        Vector3D::new(self.x + r.x, self.y + r.y, self.z + r.z)
    }
}

// Subtraction
impl Sub for Vector3D {
    type Output = Vector3D;
    fn sub(self, r: Vector3D) -> Vector3D {
        Vector3D::new(self.x - r.x, self.y - r.y, self.z - r.z)
    }
}

// Negation
impl Neg for Vector3D {
    type Output = Vector3D;
    fn neg(self) -> Vector3D {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

// Division
impl Div<Double> for Vector3D {
    type Output = Vector3D;
    fn div(self, r: Double) -> Vector3D {
        Vector3D::new(self.x / r, self.y / r, self.z / r)
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