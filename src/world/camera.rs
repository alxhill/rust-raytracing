use crate::types::{Double, Point3D, Vector3D};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    eye: Point3D,
    look_at: Point3D,
    up: Vector3D,
    exposure_time: Double,
    u: Option<Vector3D>,
    v: Option<Vector3D>,
    w: Option<Vector3D>,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            eye: Point3D::zero(),
            look_at: Point3D::zero(),
            up: Vector3D::zero(),
            exposure_time: 1.0,
            u: None,
            v: None,
            w: None,
        }
    }
}
