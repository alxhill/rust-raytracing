use crate::types::{Double, Normal, Point3D, Ray};
use crate::world::tracing::Hittable;

pub struct Plane {
    point: Point3D,
    normal: Normal,
}

pub struct Sphere {
    centre: Point3D,
    surface: Point3D
}

impl Hittable for Plane {
    fn hit(&self, ray: Ray, tmin: Double) -> crate::world::tracing::Hit {
        todo!()
    }
}