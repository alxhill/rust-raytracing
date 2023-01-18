use crate::types::{Double, Point2D, Point3D, Vector3D};
use crate::world::Ray;

pub trait Camera {
    fn ray_for_point(&self, point: &Point2D) -> Ray;
}

#[derive(Debug, Copy, Clone)]
pub struct PlanarCamera {
    direction: Vector3D,
    zw: Double,
}

#[derive(Copy, Clone, Debug)]
pub struct PerspectiveCamera {
    eye: Point3D,
    look_at: Point3D,
    up: Vector3D,
    exposure_time: Double,
    u: Option<Vector3D>,
    v: Option<Vector3D>,
    w: Option<Vector3D>,
}

impl Camera for PlanarCamera {
    fn ray_for_point(&self, point: &Point2D) -> Ray {
        let origin = Point3D::new(point.x, point.y, self.zw);
        Ray::new(origin, self.direction)
    }
}

impl PlanarCamera {
    pub fn default() -> PlanarCamera {
        PlanarCamera {
            direction: Vector3D::new(0.0, 0.0, -1.0),
            zw: 100.0,
        }
    }
}
