use crate::types::{Double, Point2D};
use crate::world::{ViewPlane, ViewXY};
use rand::prelude::*;

pub trait Sampler {
    fn points_for_pixel(&self, view_xy: &ViewXY) -> Vec<Point2D>;
}

impl Sampler for ViewPlane {
    // base implementation of one point at the center of each pixel
    fn points_for_pixel(&self, xy: &ViewXY) -> Vec<Point2D> {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (self.width as Double, self.height as Double);
        let xw = self.pixel_size * (x - 0.5 * (w - 1.0));
        let yw = self.pixel_size * (y - 0.5 * (h - 1.0));

        vec![Point2D::new(xw, yw)]
    }
}

pub struct RegularSampler {
    view_plane: ViewPlane,
    samples: u32,
}

impl RegularSampler {
    pub fn new(view_plane: ViewPlane) -> RegularSampler {
        RegularSampler {
            view_plane,
            samples: 16,
        }
    }
}

impl Sampler for RegularSampler {
    fn points_for_pixel(&self, xy: &ViewXY) -> Vec<Point2D> {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (
            self.view_plane.width as Double,
            self.view_plane.height as Double,
        );

        let mut points = Vec::new();
        let n = (self.samples as Double).sqrt();
        for i in 0..n as u32 {
            for j in 0..n as u32 {
                let xw =
                    self.view_plane.pixel_size * (x - 0.5 * (w - 1.0) + (i as Double + 0.5) / n);
                let yw =
                    self.view_plane.pixel_size * (y - 0.5 * (h - 1.0) + (j as Double + 0.5) / n);
                points.push(Point2D::new(xw, yw));
            }
        }

        points
    }
}

pub struct JitteredSampler {
    view_plane: ViewPlane,
    samples: u32,
}

impl JitteredSampler {
    pub fn new(view_plane: ViewPlane) -> JitteredSampler {
        JitteredSampler {
            view_plane,
            samples: 16,
        }
    }
}

impl Sampler for JitteredSampler {
    fn points_for_pixel(&self, xy: &ViewXY) -> Vec<Point2D> {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (
            self.view_plane.width as Double,
            self.view_plane.height as Double,
        );

        let mut points = Vec::new();
        let n = (self.samples as Double).sqrt();
        for i in 0..n as u32 {
            for j in 0..n as u32 {
                let xw = self.view_plane.pixel_size
                    * (x - 0.5 * (w - 1.0) + (i as Double + random::<Double>()) / n);
                let yw = self.view_plane.pixel_size
                    * (y - 0.5 * (h - 1.0) + (j as Double + random::<Double>()) / n);
                points.push(Point2D::new(xw, yw));
            }
        }

        points
    }
}
