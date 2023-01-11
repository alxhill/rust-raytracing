mod objects;
mod tracing;

use objects::*;
use tracing::*;

pub struct World {
    objects: Vec<Box<dyn Hittable>>
}