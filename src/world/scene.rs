use crate::types::RGBColor;
use crate::world::tracing::{Hit, Hittable};
use crate::world::{Object, Ray};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Arc<Object>>,
    pub bg_color: RGBColor,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            bg_color: RGBColor::BLACK,
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(Arc::new(object));
    }

    pub fn render_pixel(&self, ray: &Ray) -> RGBColor {
        if let Some(hit) = self.hit(ray) {
            return hit.object.unwrap().material.shade(&hit);
        }
        return self.bg_color;
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        for object in &self.objects {
            let maybe_hit = object.hit(&ray);
            match (maybe_hit, &closest_hit) {
                (Some(mut new_hit), None) => {
                    new_hit.set_obj(Arc::clone(object));
                    closest_hit = Some(new_hit)
                },
                (Some(new_hit), Some(prev_hit)) if new_hit.t < prev_hit.t => {
                    closest_hit = Some(new_hit)
                }
                (Some(_), Some(_)) | (None, _) => {}
            }
        }

        closest_hit
    }
}
