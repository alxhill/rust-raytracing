use crate::types::RGBColor;
use crate::world::tracing::{Hit, Hittable};
use crate::world::{Object, Ray};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Object>,
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
        self.objects.push(object);
    }

    pub fn render_pixel(&self, ray: &Ray) -> RGBColor {
        if let Some(hit) = self.hit(ray) {
            return hit.color;
        }
        return self.bg_color;
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        for object in &self.objects {
            let maybe_hit = object.hit(&ray);
            match (maybe_hit, closest_hit) {
                (Some(new_hit), None) => closest_hit = Some(new_hit),
                (Some(new_hit), Some(prev_hit)) if new_hit.t < prev_hit.t => {
                    closest_hit = Some(new_hit)
                }
                (Some(_), Some(_)) | (None, _) => {}
            }
        }
        closest_hit
    }
}
