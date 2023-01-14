mod camera;
mod objects;
mod ray;
mod tracing;
mod viewplane;

use crate::types::RGBColor;
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
}
