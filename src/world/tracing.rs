use crate::types::{Double, Normal, Point3D, Ray, RGBColor};

#[derive(Copy, Clone, Debug)]
pub struct Hit {
    hit_loc: Point3D,
    color: RGBColor,
    normal: Normal,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, tmin: Double) -> Option<Hit>;
}
