use crate::prelude::*;
use crate::types::{Point2D, RGBColor};

pub trait RenderTarget {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor);
}

pub trait Renderable {
    fn render_pixel(&self, pixel: &ViewXY) -> RGBColor;
}

pub struct RenderContext<'w, S: Sampler, C: Camera> {
    pub scene: &'w Scene,
    pub view_plane: &'w ViewPlane,
    pub sampler: &'w mut S,
    pub camera: &'w C,
}

impl<S: Sampler, C: Camera> Renderable for RenderContext<'_, S, C> {
    fn render_pixel(&self, pixel_xy: &ViewXY) -> RGBColor {
        let mut pixel_color = RGBColor::BLACK;
        let (w, h) = (
            self.view_plane.width as Double,
            self.view_plane.height as Double,
        );

        for _ in 0..self.sampler.num_samples() {
            let sp = self.sampler.sample_unit_square();
            let vp = Point2D::new(h, w);
            let xy_point: Point2D = pixel_xy.into();
            let pp: Point2D = (xy_point - vp * 0.5 + sp) * self.view_plane.pixel_size;

            let ray = self.camera.ray_for_point(&pp);
            pixel_color += self.scene.render_color(&ray, 0);
        }
        (pixel_color / self.sampler.num_samples()) ^ self.view_plane.inv_gamma
    }
}

pub fn render_serial<R: Renderable, T: RenderTarget>(
    view_plane: &ViewPlane,
    renderer: &mut R,
    img: &mut T,
) {
    view_plane.for_each_pixel(|xy| {
        img.set_pixel(&xy, &renderer.render_pixel(&xy));
    });
}

pub fn render_to<T: RenderTarget, S: Sampler, C: Camera>(
    scene: &Scene,
    view_plane: &ViewPlane,
    sampler: &mut S,
    camera: &C,
    img: &mut T,
) {
    let (w, h) = (view_plane.width as Double, view_plane.height as Double);

    for xy in view_plane {
        let mut pixel_color = RGBColor::BLACK;
        for _ in 0..sampler.num_samples() {
            let sp = sampler.sample_unit_square();
            let vp = Point2D::new(h, w);
            let xy_point: Point2D = (&xy).into();
            let pp: Point2D = (xy_point - vp * 0.5 + sp) * view_plane.pixel_size;

            let ray = camera.ray_for_point(&pp);
            pixel_color += scene.render_color(&ray, 0);
        }
        pixel_color = (pixel_color / sampler.num_samples()) ^ view_plane.inv_gamma;
        img.set_pixel(&xy, &pixel_color);
    }
}
