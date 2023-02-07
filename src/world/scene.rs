use crate::types::RGBColor;
use crate::world::tracing::{Hit, Hittable};
use crate::world::{Light, Object, Ray};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub bg_color: RGBColor,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            bg_color: RGBColor::BLACK,
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn render_color(&self, ray: &Ray) -> RGBColor {
        if let Some(hit) = self.hit(ray) {
            return hit.object.unwrap().material.shade(hit.clone(), &self);
        }
        self.bg_color
    }
}

impl<'t> Hittable<'t> for Scene {
    fn hit(&self, ray: &Ray) -> Option<Hit<'t>> {
        let mut closest_hit: Option<Hit> = None;
        for object in &self.objects {
            let maybe_hit = object.hit(ray);
            match (maybe_hit, &closest_hit) {
                (Some(mut new_hit), None) => {
                    new_hit.set_obj(object);
                    closest_hit = Some(new_hit)
                },
                (Some(mut new_hit), Some(prev_hit)) if new_hit.t < prev_hit.t => {
                    new_hit.set_obj(object);
                    closest_hit = Some(new_hit)
                }
                (Some(_), Some(_)) | (None, _) => {}
            }
        }

        closest_hit
    }
}
