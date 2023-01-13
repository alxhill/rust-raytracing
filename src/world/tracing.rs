use crate::types::{Double, Point3D, Ray, RGBColor, Vector3D};
use crate::world::*;

#[derive(Copy, Clone, Debug)]
pub struct Hit {
    hit_loc: Point3D,
    normal: Vector3D,
    pub color: RGBColor,
}

impl Hit {
    pub fn hit(hit_loc: Point3D, normal: Vector3D, color: RGBColor) -> Hit {
        Hit {
            hit_loc, normal, color
        }
    }

    pub const epsilon: Double = 1e-6;
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: &mut Double) -> Option<Hit>;
}

