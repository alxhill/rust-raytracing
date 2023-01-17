use crate::types::{Double, Point3D, Vector3D};
use crate::world::viewplane::ViewXY;
use crate::world::{Ray, ViewPlane};

#[derive(Debug, Copy, Clone)]
pub struct PlanarCamera {
    view_plane: ViewPlane,
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

pub trait Camera {
    fn get_ray(&self, view_xy: &ViewXY) -> Ray;
}

const DEFAULT_DIRECTION: Vector3D = Vector3D::new(0.0, 0.0, -1.0);
const ZW: Double = 100.0;

impl Camera for PlanarCamera {
    fn get_ray(&self, xy: &ViewXY) -> Ray {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (
            self.view_plane.width as Double,
            self.view_plane.height as Double,
        );
        let xw = self.view_plane.pixel_size * (x - 0.5 * (w - 1.0));
        let yw = self.view_plane.pixel_size * (y - 0.5 * (h - 1.0));

        Ray::new(Point3D::new(xw, yw, ZW), DEFAULT_DIRECTION)
    }
}

impl PlanarCamera {
    pub fn default(view_plane: ViewPlane) -> PlanarCamera {
        PlanarCamera { view_plane }
    }
}
