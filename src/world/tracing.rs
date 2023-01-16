use crate::types::{Double, Point3D, RGBColor, Vector3D};
use crate::world::*;

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

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub trait Tracer {
    fn trace(&self, ray: &Ray, depth: u32) -> RGBColor;
}
