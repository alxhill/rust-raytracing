use crate::types::{Double, Normal, Point3D, Ray, RGBColor};

struct Hit {
    hit_loc: Point3D,
    color: RGBColor,
    normal: Normal,
}

trait Hittable {
    fn hit(ray: Ray, tmin: Double) -> Hit;
}