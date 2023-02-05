use crate::types::RGBColor;
use crate::world::tracing::{Hit, Hittable};
use crate::world::{Light, Object, Ray};
use std::fmt::Debug;
use bumpalo::Bump;
use bumpalo::collections::Vec;

#[derive(Debug)]
pub struct Scene<'w> {
    pub objects: Vec<'w, & 'w Object<'w>>,
    pub lights: Vec<'w, &'w Light>,
    pub bg_color: RGBColor,
}

impl<'w> Scene<'w> {
    pub fn new(arena: &'w Bump) -> &mut Scene<'w> {
        arena.alloc(Scene {
            objects: Vec::new_in(arena),
            lights: Vec::new_in(arena),
            bg_color: RGBColor::BLACK,
        })
    }

    pub fn add_object(&mut self, object: &'w Object<'w>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: &'w Light) {
        self.lights.push(light);
    }

    pub fn render_color(&self, ray: &Ray) -> RGBColor {
        if let Some(hit) = self.hit(ray) {
            return hit.object.unwrap().material.shade(hit.clone(), &self);
        }
        self.bg_color
    }
}

impl<'t> Hittable<'t> for Scene<'t> {
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
