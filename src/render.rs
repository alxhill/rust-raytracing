use crate::types::RGBColor;

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
        let points = sampler.points_for_pixel(xy);
        for point in points.iter() {
            let ray = camera.ray_for_point(point);
            pixel_color += scene.render_color(&ray, 0);
        }
        pixel_color = (pixel_color / points.len()) ^ view_plane.inv_gamma;
        img.set_pixel(xy, &pixel_color);
    });
}
