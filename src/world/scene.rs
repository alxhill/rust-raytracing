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
        if let (Some(hit), Some(obj)) = self.hit(ray) {
            return obj.material.shade(hit, self);
        }
        self.bg_color
    }

    fn hit<'t>(&'t self, ray: &'t Ray) -> (Option<Hit>, Option<&'t Object>) {
        let mut closest_hit = None;
        let mut closest_obj = None;
        for object in &self.objects {
            let maybe_hit = object.hit(ray);
            match (maybe_hit, &closest_hit) {
                (Some(new_hit), None) => {
                    closest_obj = Some(object);
                    closest_hit = Some(new_hit)
                },
                (Some(new_hit), Some(prev_hit)) if new_hit.t < prev_hit.t => {
                    closest_obj = Some(object);
                    closest_hit = Some(new_hit)
                }
                (Some(_), Some(_)) | (None, _) => {}
            }
        }
        (closest_hit, closest_obj)
    }
}
