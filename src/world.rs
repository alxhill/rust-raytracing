mod objects;
mod tracing;
mod camera;


use tracing::*;
pub use camera::*;
pub use objects::*;

pub struct World {
    camera: Camera,
    objects: Vec<Box<dyn Hittable>>
}

impl World {
    pub fn new() -> World {
        World {camera: Camera::default(), objects: Vec::new()}
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj)
    }

    pub fn objects(&self) -> &Vec<Box<dyn Hittable>> {
        &self.objects
    }
}
