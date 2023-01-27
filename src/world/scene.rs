use crate::types::RGBColor;
use crate::world::tracing::{Hit, Hittable};
use crate::world::{AmbientLight, Light, Object, Ray};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Arc<Object>>,
    ambient_light: AmbientLight,
    lights: Vec<Arc<Light>>,
    pub bg_color: RGBColor,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            ambient_light: AmbientLight::default(),
            lights: Vec::new(),
            bg_color: RGBColor::BLACK,
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(Arc::new(object));
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(Arc::new(light));
    }

    pub fn render_color(&self, ray: &Ray) -> RGBColor {
        if let Some(hit) = self.hit(ray) {
            let hit2 = hit.clone();
            return hit.object.unwrap().material.shade(hit2);
        }
        self.bg_color
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
                (Some(mut new_hit), Some(prev_hit)) if new_hit.t < prev_hit.t => {
                    new_hit.set_obj(Arc::clone(object));
                    closest_hit = Some(new_hit)
                }
                (Some(_), Some(_)) | (None, _) => {}
            }
        }

        closest_hit
    }
}
