use crate::types::{Double, RGBColor, Vector3D};
use crate::world::{Hit, Light};
use std::fmt::Debug;
use bumpalo::collections::Vec;

pub trait Shadeable: Debug {
    fn shade(&self, hit: Hit, lights: &Vec<&Light>) -> RGBColor;
}

pub trait BRDF: Debug {
    fn f(&self, hit: &Hit, wi: &Vector3D, wo: &Vector3D) -> RGBColor;
    fn rho(&self, hit: &Hit, wo: &Vector3D) -> RGBColor;
}

#[derive(Debug, Copy, Clone)]
pub struct Matte<T: BRDF> {
    ambient: T,
    diffuse: T,
}

impl Matte<Lambertian> {
    #[inline(always)]
    pub fn new(ka: Double, kd: Double, cd: RGBColor) -> Matte<Lambertian> {
        Matte {
            ambient: Lambertian::new(ka, cd),
            diffuse: Lambertian::new(kd, cd)
        }
    }
}

impl<T: BRDF> Shadeable for Matte<T> {
    fn shade(&self, hit: Hit, lights: &Vec<&Light>) -> RGBColor {
        let wo = -hit.ray.direction;
        let mut l = self.ambient.rho(&hit, &wo);
        for light in lights.iter() {
            let wi = light.direction(&hit.hit_loc);
            let ndotwi = hit.normal * wi;
            if ndotwi > 0.0 {
                l += self.diffuse.f(&hit, &wo, &wi) * light.l() * ndotwi;
            }
        }
        l
    }
}

// for initial implementation
impl Shadeable for RGBColor {
    fn shade(&self, _hit: Hit, _lights: &Vec<&Light>) -> RGBColor {
        *self
    }
}

#[derive(Debug)]
pub struct Lambertian {
    kd: Double,
    color: RGBColor,
}

impl Lambertian {
    pub fn new(kd: Double, color: RGBColor) -> Lambertian {
        Lambertian { kd, color }
    }
}

impl BRDF for Lambertian {
    fn f(&self, _hit: &Hit, _wi: &Vector3D, _wo: &Vector3D) -> RGBColor {
        self.color * self.kd / std::f64::consts::PI
    }

    fn rho(&self, _hit: &Hit, _wo: &Vector3D) -> RGBColor {
        self.color * self.kd
    }
}