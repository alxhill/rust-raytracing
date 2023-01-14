mod camera;
mod objects;
mod ray;
mod tracing;
mod viewplane;

use crate::types::RGBColor;
pub use camera::*;
pub use objects::*;
pub use ray::*;
use tracing::*;

pub struct World {
    camera: Camera,
    objects: Vec<Box<dyn Hittable>>,
    pub bg_color: RGBColor,
}

impl World {
    pub fn new() -> World {
        World {
            camera: Camera::default(),
            objects: Vec::new(),
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
