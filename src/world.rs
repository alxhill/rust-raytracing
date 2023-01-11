mod objects;
mod tracing;

use tracing::*;
pub use objects::*;

pub struct World {
    objects: Vec<Box<dyn Hittable>>
}

impl World {
    pub fn new() -> World {
        World {objects: Vec::new()}
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj)
    }
}