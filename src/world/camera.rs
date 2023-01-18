use crate::types::{Double, Point2D, Point3D, Vector3D};
use crate::world::viewplane::ViewXY;
use crate::world::{Ray, ViewPlane};

pub struct RegularSampler;

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

pub trait Camera {
    fn ray_for_point(&self, point: &Point2D) -> Ray;
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

pub trait PointMapper {
    fn points_for_pixel(&self, view_xy: &ViewXY) -> Vec<Point2D>;
}

impl PointMapper for ViewPlane {
    // base implementation of one point at the center of each pixel
    fn points_for_pixel(&self, xy: &ViewXY) -> Vec<Point2D> {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (self.width as Double, self.height as Double);
        let xw = self.pixel_size * (x - 0.5 * (w - 1.0));
        let yw = self.pixel_size * (y - 0.5 * (h - 1.0));

        vec![Point2D::new(xw, yw)]
    }
}
