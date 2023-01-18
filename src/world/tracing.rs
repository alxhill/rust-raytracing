use crate::types::{Double, Point3D, RGBColor, Vector3D};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Hit {
    pub t: Double,
    hit_loc: Point3D,
    normal: Vector3D,
    pub color: RGBColor,
}

impl Hit {
    pub fn hit(dist: Double, hit_loc: Point3D, normal: Vector3D, color: RGBColor) -> Hit {
        Hit {
            t: dist,
            hit_loc,
            normal,
            color,
        }
    }

    pub const EPSILON: Double = 1e-6;
}

pub trait Hittable: Debug {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
