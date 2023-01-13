use crate::types::{Double, Float, Normal, Point3D, Ray, RGBColor};
use crate::world::tracing::{Hit, Hittable};

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    point: Point3D,
    normal: Normal,
    color: RGBColor, // temporary
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    origin: Point3D,
    radius: Float,
    color: RGBColor, // temporary
}

impl Sphere {
    pub fn new(origin: Point3D, radius: Float, color: RGBColor) -> Sphere {
        Sphere {
            origin, radius, color
        }
    }
}

impl Plane {
    pub fn new(point: Point3D, normal: Normal, color: RGBColor) -> Plane {
        Plane {
            point, normal, color
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: Ray, tmin: Double) -> Option<Hit> {
        todo!()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, tmin: Double) -> Option<Hit> {
        todo!()
    }
}
