use crate::types::Point3D;
use crate::types::vector3d::Vector3D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3D
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray {origin, direction}
    }
}