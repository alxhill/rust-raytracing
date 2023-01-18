mod camera;
mod objects;
mod sampler;
mod scene;
mod tracing;
mod viewplane;

use crate::render::Renderable;
use crate::types::RGBColor;
pub use crate::world::viewplane::*;
pub use camera::*;
pub use objects::*;
pub use sampler::*;
pub use scene::*;
use tracing::*;

pub struct World<C: Camera, S: Sampler> {
    camera: C,
    scene: Scene,
    sampler: S,
    view_plane: ViewPlane,
    pub bg_color: RGBColor,
}

impl World<PlanarCamera, ViewPlane> {
    pub fn new<S: Sampler>(
        scene: Scene,
        view_plane: ViewPlane,
        sampler: S,
    ) -> World<PlanarCamera, S> {
        let w = World {
            camera: PlanarCamera::default(),
            scene,
            sampler: sampler,
            view_plane: view_plane,
            bg_color: RGBColor::BLACK,
        };
        return w;
    }
}

impl<C: Camera, S: Sampler> World<C, S> {
    pub fn render_to<T: Renderable>(&self, img: &mut T) {
        self.view_plane.for_each_pixel(|xy| {
            let mut pixel_color = RGBColor::BLACK;
            let points = self.sampler.points_for_pixel(&xy);
            for point in points.iter() {
                let ray = self.camera.ray_for_point(&point);
                pixel_color += self.render_pixel(&ray);
            }
            pixel_color = pixel_color / points.len() ^ self.view_plane.inv_gamma;
            img.set_pixel(&xy, &pixel_color);
        });
    }

    fn render_pixel(&self, ray: &Ray) -> RGBColor {
        if let Some(hit) = self.scene.hit(ray) {
            return hit.color;
        }
        return self.scene.bg_color;
    }
}
