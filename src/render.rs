use crate::prelude::*;
use crate::types::{Point2D, RGBColor};

pub trait RenderTarget {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor);
}

pub fn render_to<'w, T: RenderTarget, S: Sampler, C: Camera>(
    scene: &'w Scene,
    view_plane: &ViewPlane,
    sampler: &mut S,
    camera: &C,
    img: &mut T,
) {
    let (w, h) = (view_plane.width as Double, view_plane.height as Double);

    view_plane.for_each_pixel(|xy| {
        let mut pixel_color = RGBColor::BLACK;
        for _ in 0..sampler.num_samples() {
            let sp = sampler.sample_unit_square();
            let vp = Point2D::new(h, w);
            let xy_point: Point2D = xy.into();
            let pp: Point2D = (xy_point - vp * 0.5 + sp) * view_plane.pixel_size;

            let ray = camera.ray_for_point(&pp);
            pixel_color += scene.render_color(&ray, 0);
        }
        pixel_color = (pixel_color / sampler.num_samples()) ^ view_plane.inv_gamma;
        img.set_pixel(xy, &pixel_color);
    });
}
