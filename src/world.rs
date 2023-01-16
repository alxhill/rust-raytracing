mod camera;
mod objects;
mod ray;
mod tracing;
mod viewplane;

use crate::render::Renderable;
use crate::types::Double;
use crate::types::RGBColor;
pub use crate::world::viewplane::*;
pub use camera::*;
pub use objects::*;
pub use ray::*;
use tracing::*;

pub struct World {
    camera: FlatCamera,
    objects: Vec<Box<dyn Hittable>>,
    view_plane: ViewPlane,
    pub bg_color: RGBColor,
}

impl World {
    pub fn new() -> World {
        World {
            camera: FlatCamera::default(),
            objects: Vec::new(),
            view_plane: ViewPlane::new(128, 128, 1.0),
            bg_color: RGBColor::BLACK,
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj)
    }

    pub fn objects(&self) -> &Vec<Box<dyn Hittable>> {
        &self.objects
    }

    pub fn render_to<T: Renderable>(&self, img: &mut T) {
        self.view_plane.for_each_pixel(|xy| {
            let ray = self.camera.get_ray(&xy, &self.view_plane);
            let color = self.render_pixel(&ray);
            img.set_pixel(xy, &color);
        });
    }

    fn render_pixel(&self, ray: &Ray) -> RGBColor {
        let mut tmin: Double = f64::MAX;
        let mut hit: Option<Hit> = None;
        for obj in self.objects() {
            let maybe_hit = obj.hit(&ray, &mut tmin);
            match maybe_hit {
                Some(h) => hit = Some(h),
                None => continue,
            }
        }
        match hit {
            Some(h) => h.color,
            None => self.bg_color,
        }
    }
}
