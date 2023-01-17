use crate::types::{Double, Point3D, Vector3D};
use crate::world::viewplane::ViewXY;
use crate::world::{Ray, ViewPlane};

pub struct RegularSampler;

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

pub trait RayCaster {
    fn cast_ray(&self, origin: &Point3D, view_xy: &ViewXY) -> Ray;
}

pub trait RaySampler<T: RayCaster> {
    fn rays_for_pixel(&self, view_xy: &ViewXY) -> Vec<Ray>;
}

const DEFAULT_DIRECTION: Vector3D = Vector3D::new(0.0, 0.0, -1.0);
const ZW: Double = 100.0;

impl RayCaster for PlanarCamera {
    fn cast_ray(&self, origin: &Point3D, _xy: &ViewXY) -> Ray {
        Ray::new(*origin, DEFAULT_DIRECTION)
    }
}

impl<T: RayCaster> RaySampler<T> for RegularSampler {
    fn rays_for_pixel(&self, xy: &ViewXY) -> Vec<Ray> {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (
            self.view_plane.width as Double,
            self.view_plane.height as Double,
        );
        let xw = self.view_plane.pixel_size * (x - 0.5 * (w - 1.0));
        let yw = self.view_plane.pixel_size * (y - 0.5 * (h - 1.0));

        let origin = Point3D::new(xw, yw, ZW);
        vec![self.cast_ray(&origin, xy)]
    }
}

impl PlanarCamera {
    pub fn default(view_plane: ViewPlane) -> PlanarCamera {
        PlanarCamera { view_plane }
    }
}
