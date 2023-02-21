use crate::Double;
use crate::types::{Point2D, RGBColor};

use crate::world::{Camera, Sampler, Scene, ViewPlane, ViewXY};

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
    let (w, h) = (
        view_plane.width as Double,
        view_plane.height as Double,
    );

    view_plane.for_each_pixel(|xy| {
        let mut pixel_color = RGBColor::BLACK;
        for _ in 0..sampler.num_samples() {
            let sp = sampler.sample_unit_square();
            let xw = view_plane.pixel_size * (xy.x() as Double - 0.5 * h + sp.x);
            let yw = view_plane.pixel_size * (xy.y() as Double - 0.5 * w + sp.y);

            let point = Point2D::new(xw, yw);

            let ray = camera.ray_for_point(&point);
            pixel_color += scene.render_color(&ray, 0);
        }
        pixel_color = (pixel_color / sampler.num_samples()) ^ view_plane.inv_gamma;
        img.set_pixel(xy, &pixel_color);
    });
}
