use crate::types::RGBColor;

mod canvas;
mod image;

pub use crate::render::canvas::CanvasTarget;
pub use crate::render::image::ImageTarget;
use crate::world::{Camera, Sampler, Scene, ViewPlane, ViewXY};

pub trait RenderTarget {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor);
}

pub fn render_to<T: RenderTarget, S: Sampler, C: Camera>(
    scene: Scene,
    view_plane: &ViewPlane,
    sampler: &S,
    camera: &C,
    img: &mut T,
) {
    view_plane.for_each_pixel(|xy| {
        let mut pixel_color = RGBColor::BLACK;
        let points = sampler.points_for_pixel(&xy);
        for point in points.iter() {
            let ray = camera.ray_for_point(&point);
            pixel_color += scene.render_color(&ray);
        }
        pixel_color = pixel_color / points.len() ^ view_plane.inv_gamma;
        img.set_pixel(&xy, &pixel_color);
    });
}
