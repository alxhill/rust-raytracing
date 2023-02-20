use crate::types::{Point2D, RGBColor};

use crate::world::{Camera, Sampler, Scene, ViewPlane, ViewXY};

pub trait RenderTarget {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor);
}

pub fn render_to<'w, T: RenderTarget, S: Sampler, C: Camera>(
    scene: &'w Scene,
    view_plane: &ViewPlane,
    sampler: &S,
    camera: &C,
    img: &mut T,
) {
    view_plane.for_each_pixel(|xy| {
        let mut pixel_color = RGBColor::BLACK;
        let mut points = Vec::with_capacity(sampler.num_samples() as usize);
        for _ in 0..sampler.num_samples() {
            points.push(sampler.sample_unit_square());
        }
        for point in points.iter() {
            let ray = camera.ray_for_point(point);
            pixel_color += scene.render_color(&ray, 0);
        }
        pixel_color = (pixel_color / points.len()) ^ view_plane.inv_gamma;
        img.set_pixel(xy, &pixel_color);
    });
}
