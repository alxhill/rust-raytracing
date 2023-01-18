mod camera;
mod objects;
mod ray;
mod scene;
mod tracing;
mod viewplane;

use crate::render::Renderable;
use crate::types::RGBColor;
pub use crate::world::viewplane::*;
pub use camera::*;
pub use objects::*;
pub use ray::*;
pub use scene::*;
use std::rc::Rc;
use tracing::*;

pub struct World<C: Camera, P: PointMapper> {
    camera: C,
    scene: Scene,
    point_mapper: Rc<P>,
    view_plane: Rc<ViewPlane>,
    pub bg_color: RGBColor,
}

impl World<PlanarCamera, ViewPlane> {
    pub fn new(scene: Scene) -> World<PlanarCamera, ViewPlane> {
        let view_plane = Rc::new(ViewPlane::new(128, 128, 1.0));
        let w = World {
            camera: PlanarCamera::default(),
            scene,
            point_mapper: view_plane.clone(),
            view_plane: view_plane.clone(),
            bg_color: RGBColor::BLACK,
        };
        return w;
    }
}

impl<C: Camera, P: PointMapper> World<C, P> {
    pub fn render_to<T: Renderable>(&self, img: &mut T) {
        self.view_plane.for_each_pixel(|xy| {
            let mut pixel_color = RGBColor::BLACK;
            let points = self.point_mapper.points_for_pixel(&xy);
            for point in points.iter() {
                let ray = self.camera.ray_for_point(&point);
                // add the normalised color using gamma
                pixel_color += self.render_pixel(&ray) ^ self.view_plane.inv_gamma;
            }
            pixel_color = pixel_color / points.len();
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
