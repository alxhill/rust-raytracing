mod camera;
mod objects;
mod ray;
mod tracing;
mod viewplane;

use crate::render::Renderable;
use crate::types::RGBColor;
pub use crate::world::viewplane::*;
pub use camera::*;
pub use objects::*;
pub use ray::*;
use tracing::*;

pub struct World<C: RayCaster, R: RaySampler<C>> {
    sampler: R,
    camera: C,
    objects: Vec<Box<dyn Hittable>>,
    view_plane: ViewPlane,
    pub bg_color: RGBColor,
}

impl World<PlanarCamera, PlanarCamera> {
    pub fn default() -> World<PlanarCamera, PlanarCamera> {
        let view_plane = ViewPlane::new(128, 128, 1.0);
        let camera = PlanarCamera::default(view_plane);
        let w = World {
            sampler: camera,
            camera,
            objects: Vec::new(),
            view_plane,
            bg_color: RGBColor::BLACK,
        };
        return w;
    }
}

impl<C: RayCaster, R: RaySampler<C>> World<C, R> {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj)
    }

    pub fn objects(&self) -> &Vec<Box<dyn Hittable>> {
        &self.objects
    }

    pub fn render_to<T: Renderable>(&self, img: &mut T) {
        self.view_plane.for_each_pixel(|xy| {
            let rays = self.sampler.rays_for_pixel(&xy);
            for ray in rays {
                // normalise the color using gamma
                let color = self.render_pixel(&ray) ^ self.view_plane.inv_gamma;
                img.set_pixel(xy, &color);
            }
        });
    }

    fn render_pixel(&self, ray: &Ray) -> RGBColor {
        let mut curr_hit: Option<Hit> = None;
        for obj in self.objects() {
            let maybe_hit = obj.hit(&ray);
            match (maybe_hit, curr_hit) {
                (Some(new_hit), None) => curr_hit = Some(new_hit),
                (Some(new_hit), Some(prev_hit)) if new_hit.t < prev_hit.t => {
                    curr_hit = Some(new_hit)
                }
                (Some(_), Some(_)) | (None, _) => {}
            }
        }
        match curr_hit {
            Some(h) => h.color,
            None => self.bg_color,
        }
    }
}
