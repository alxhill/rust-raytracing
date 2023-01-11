use crate::types::{Double, Float, Normal, Point3D, Ray};
use crate::world::tracing::{Hit, Hittable};

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    point: Point3D,
    normal: Normal,
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    origin: Point3D,
    radius: Float,
}

impl Sphere {
    pub fn new(origin: Point3D, radius: Float) -> Sphere {
        Sphere {
            origin, radius
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: Ray, tmin: Double) -> crate::world::tracing::Hit {
        todo!()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, tmin: Double) -> Hit {
        todo!()
    }
}
