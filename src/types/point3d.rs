use crate::types;
use crate::types::{Distance, Double};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3D {
    x: Double,
    y: Double,
    z: Double,
}

impl Point3D {
    pub fn new(x: Double, y: Double, z: Double) -> Point3D {
        Point3D { x, y, z }
    }
}

impl Distance for Point3D {
    fn dist(&self, other: Point3D) -> Double {
        (types::diff_sq(self.x, other.x) + types::diff_sq(self.y, other.y) + types::diff_sq(self.z, other.z)).sqrt()
    }
}
