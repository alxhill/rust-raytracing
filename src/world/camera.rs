use crate::types::{Double, Point3D, Vector3D};
use crate::world::viewplane::ViewXY;
use crate::world::{Ray, ViewPlane};

#[derive(Copy, Clone, Debug)]
pub struct FlatCamera {
    eye: Point3D,
    look_at: Point3D,
    up: Vector3D,
    exposure_time: Double,
    u: Option<Vector3D>,
    v: Option<Vector3D>,
    w: Option<Vector3D>,
}

pub trait Camera {
    fn get_ray(&self, view_xy: &ViewXY, view_plane: &ViewPlane) -> Ray;
}

const DEFAULT_DIRECTON: Vector3D = Vector3D::new(0.0, 0.0, -1.0);
const ZW: Double = 100.0;

impl Camera for FlatCamera {
    fn get_ray(&self, xy: &ViewXY, view_plane: &ViewPlane) -> Ray {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (view_plane.width as Double, view_plane.height as Double);
        let xw = view_plane.pixel_size * (x - 0.5 * (w as Double - 1.0));
        let yw = view_plane.pixel_size * (y - 0.5 * (h as Double - 1.0));

        Ray::new(Point3D::new(xw, yw, ZW), DEFAULT_DIRECTON)
    }
}

impl FlatCamera {
    pub fn default() -> FlatCamera {
        FlatCamera {
            eye: Point3D::new(-2.0, 2.0, 1.0),
            look_at: Point3D::zero(),
            up: Vector3D::zero(),
            exposure_time: 1.0,
            u: None,
            v: None,
            w: None,
        }
    }
}
