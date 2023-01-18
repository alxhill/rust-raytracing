use crate::types::{Double, Point2D, Point3D, Vector3D};
use crate::world::Ray;

pub trait Camera {
    fn ray_for_point(&self, point: &Point2D) -> Ray;
    fn move_by(&mut self, dir: Vector3D);
}

#[derive(Debug, Copy, Clone)]
pub struct OrthoCamera {
    direction: Vector3D,
    zw: Double,
}

impl OrthoCamera {
    pub fn default() -> OrthoCamera {
        OrthoCamera {
            direction: Vector3D::new(0.0, 0.0, -1.0),
            zw: 100.0,
        }
    }
}

impl Camera for OrthoCamera {
    fn ray_for_point(&self, point: &Point2D) -> Ray {
        let origin = Point3D::new(point.x, point.y, self.zw);
        Ray::new(origin, self.direction)
    }

    fn move_by(&mut self, dir: Vector3D) {
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PerspectiveCamera {
    eye: Point3D,
    distance: Double,
}

impl PerspectiveCamera {
    pub fn default() -> PerspectiveCamera {
        PerspectiveCamera::new(-100.0, 100.0)
    }

    pub fn new(eye_dist: Double, plane_dist: Double) -> PerspectiveCamera {
        PerspectiveCamera {
            eye: Point3D::new(0.0, 0.0, eye_dist),
            distance: plane_dist,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn ray_for_point(&self, point: &Point2D) -> Ray {
        Ray::new(
            self.eye,
            Vector3D::new(point.x, point.y, self.distance).normalize(),
        )
    }

    fn move_by(&mut self, dir: Vector3D) {
        self.eye = self.eye + dir;
    }
}
