use crate::types::RGBColor;
use crate::world::tracing::{Hit, Hittable};
use crate::world::{Light, Object, Ray};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub type Depth = u8;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub bg_color: RGBColor,
    max_depth: Depth,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            bg_color: RGBColor::BLACK,
            max_depth: 2,
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn render_color(&self, ray: &Ray, depth: Depth) -> RGBColor {
        if depth > self.max_depth {
            return self.bg_color;
        }

        if let Some((hit, obj)) = self.hit(ray) {
            return obj.material.shade(hit, self, depth);
        }
        self.bg_color
    }

    fn hit(&self, ray: &Ray) -> Option<(Hit, &Object)> {
        let mut result: Option<(Hit, &Object)> = None;
        for object in &self.objects {
            let maybe_hit = object.hit(ray);
            match (maybe_hit, &result) {
                (Some(new_hit), None) => {
                    result = Some((new_hit, object));
                }
                (Some(new_hit), Some((prev_hit, _))) if new_hit.t < prev_hit.t => {
                    result = Some((new_hit, object));
                }
                _ => {}
            }
        }
        result
    }
}
