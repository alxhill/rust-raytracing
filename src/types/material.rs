use crate::types::{Double, RGBColor, Vector3D};
use crate::world::Hit;
use std::fmt::Debug;

pub trait BRDF: Debug {
    fn f(&self, hit: &Hit, wi: &Vector3D, wo: &Vector3D) -> RGBColor;
    fn rho(&self, hit: &Hit, wo: &Vector3D) -> RGBColor;
}

// for early implementation
impl BRDF for RGBColor {
    fn f(&self, _hit: &Hit, _wi: &Vector3D, _wo: &Vector3D) -> RGBColor {
        self.clone()
    }

    fn rho(&self, _hit: &Hit, _wo: &Vector3D) -> RGBColor {
        self.clone()
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