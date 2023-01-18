use crate::types::{Double, Point2D, Point3D, Vector3D};
use crate::world::Ray;

#[derive(Copy, Clone, Debug)]
pub struct CameraPosition {
    eye: Point3D,
    look_at: Point3D,
    up: Vector3D,
    distance: Double,
    uvw: (Vector3D, Vector3D, Vector3D),
}

impl CameraPosition {
    pub fn new(eye: Point3D, look_at: Point3D, up: Vector3D, distance: Double) -> CameraPosition {
        let w = (eye - look_at).normalize();
        let u = (up ^ w).normalize();
        let v = w ^ u;
        CameraPosition {
            eye,
            look_at,
            up,
            distance,
            uvw: (u, v, w),
        }
    }

    pub fn move_by(&mut self, dir: &Vector3D) {
        self.eye = self.eye + *dir;
        self.update_uvw();
    }

    fn update_uvw(&mut self) {
        let w = (self.eye - self.look_at).normalize();
        let u = (self.up ^ w).normalize();
        let v = w ^ u;
        self.uvw = (u, v, w);
    }
}

pub trait Camera {
    fn ray_for_point(&self, point: &Point2D) -> Ray;
    fn position(&mut self) -> &mut CameraPosition;
}

#[derive(Debug, Copy, Clone)]
pub struct OrthoCamera {
    position: CameraPosition,
}

impl OrthoCamera {
    pub fn default() -> OrthoCamera {
        OrthoCamera {
            position: CameraPosition::new(
                Point3D::new(0.0, 0.0, 100.0),
                Point3D::new(0.0, 0.0, 0.0),
                Vector3D::new(0.0, 1.0, 0.0),
                100.0,
            ),
        }
    }
}

impl Camera for OrthoCamera {
    fn ray_for_point(&self, point: &Point2D) -> Ray {
        let origin = self.position.eye + Point3D::new(point.x, point.y, 0.0);
        let direction = (self.position.look_at - self.position.eye).normalize();
        Ray::new(origin, direction)
    }

    fn position(&mut self) -> &mut CameraPosition {
        &mut self.position
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PerspectiveCamera {
    position: CameraPosition,
}

impl PerspectiveCamera {
    pub fn default() -> PerspectiveCamera {
        PerspectiveCamera::new(-100.0, 100.0)
    }

    pub fn new(eye_dist: Double, plane_dist: Double) -> PerspectiveCamera {
        PerspectiveCamera {
            position: CameraPosition::new(
                Point3D::new(0.0, 0.0, eye_dist),
                Point3D::new(0.0, 0.0, 0.0),
                Vector3D::UP,
                plane_dist,
            ),
        }
    }
}

impl Camera for PerspectiveCamera {
    fn ray_for_point(&self, point: &Point2D) -> Ray {
        Ray::new(
            self.position.eye,
            Vector3D::new(point.x, point.y, self.position.distance).normalize(),
        )
    }

    fn position(&mut self) -> &mut CameraPosition {
        &mut self.position
    }
}
