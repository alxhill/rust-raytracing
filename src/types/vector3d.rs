use crate::types;
use crate::types::{Distance, Double, utils};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3D {
    x: Double,
    y: Double,
    z: Double,
}

impl Vector3D {
    pub fn zero() -> Vector3D {
        Vector3D { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn new(x: Double, y: Double, z: Double) -> Vector3D {
        Vector3D { x, y, z }
    }
}

impl Distance for Vector3D {
    fn dist(&self, other: &Vector3D) -> Double {
        (utils::diff_sq(self.x, other.x) + utils::diff_sq(self.y, other.y) + utils::diff_sq(self.z, other.z)).sqrt()
    }
}
