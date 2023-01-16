mod camera;
mod objects;
mod ray;
mod tracing;
mod viewplane;

use crate::render::Renderable;
use crate::types::{Double, Point3D, RGBColor, Vector3D};
use crate::world::viewplane::ViewPlane;
pub use camera::*;
pub use objects::*;
pub use ray::*;
use tracing::*;

pub struct World {
    camera: Camera,
    objects: Vec<Box<dyn Hittable>>,
    view_plane: ViewPlane,
    pub bg_color: RGBColor,
}

impl World {
    pub fn new() -> World {
        World {
            camera: Camera::default(),
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
        for x in 0..self.view_plane.hres {
            for y in 0..self.view_plane.vres {
                img.set_pixel(x, y, self.render_pixel(x, y));
            }
        }
    }

    fn render_pixel(&self, x: u32, y: u32) -> RGBColor {
        let mut tmin: Double = f64::MAX;
        let ray: Ray = Ray::new(
            Point3D::new(x as Double, y as Double, 100.0),
            Vector3D::new(0.0, 0.0, -1.0),
        );
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
