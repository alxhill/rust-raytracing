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
                Point3D { x: 0.0, y: 0.0, z: 100.0 },
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Vector3D { x: 0.0, y: 1.0, z: 0.0 },
                100.0,
            ),
        }
    }
}

impl Camera for OrthoCamera {
    fn ray_for_point(&self, point: &Point2D) -> Ray {
        let x = point.x;
        let y = point.y;
        let origin = self.position.eye + Point3D { x, y, z: 0.0 };
        let direction = (self.position.look_at - self.position.eye).normalize();
        Ray::new(origin, direction)
    }

    fn position(&mut self) -> &mut CameraPosition {
        &mut self.position
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PinholeCamera {
    position: CameraPosition,
}

impl PinholeCamera {
    pub fn default() -> PinholeCamera {
        PinholeCamera::new(-100.0, 100.0)
    }

    pub fn new(eye_dist: Double, plane_dist: Double) -> PinholeCamera {
        PinholeCamera {
            position: CameraPosition::new(
                Point3D { x: 0.0, y: 0.0, z: eye_dist },
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Vector3D::UP,
                plane_dist,
            ),
        }
    }
}

impl Camera for PinholeCamera {
    fn ray_for_point(&self, point: &Point2D) -> Ray {
        let x = point.x;
        let y = point.y;
        let z = self.position.distance;
        Ray::new(
            self.position.eye,
            Vector3D { x, y, z }.normalize(),
        )
    }

    fn position(&mut self) -> &mut CameraPosition {
        &mut self.position
    }
}
