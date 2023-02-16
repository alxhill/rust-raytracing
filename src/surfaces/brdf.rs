use std::fmt::Debug;
use num_traits::Pow;
use crate::types::{Double, RGBColor, Vector3D};
use crate::world::Hit;

pub trait BRDF: Debug {
    fn f(&self, hit: &Hit, wi: &Vector3D, wo: &Vector3D) -> RGBColor;
    fn rho(&self, hit: &Hit, wo: &Vector3D) -> RGBColor;
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct Glossy {
    ks: Double,
    exp: Double,
    color: RGBColor
}

impl Glossy {
    pub fn new(ks: Double, exp: Double, color: RGBColor) -> Glossy {
        Glossy {ks, exp, color}
    }
}

impl BRDF for Glossy {
    fn f(&self, hit: &Hit, wi: &Vector3D, wo: &Vector3D) -> RGBColor {
        let ndotwi = hit.normal * *wi;
        let r = -*wi + 2.0 * hit.normal * ndotwi;
        let rdotwo = r * *wo;

        if rdotwo > 0.0 {
            self.color * self.ks * rdotwo.pow(self.exp)
        } else {
            RGBColor::BLACK
        }

    }

    fn rho(&self, _hit: &Hit, _wo: &Vector3D) -> RGBColor {
        RGBColor::BLACK
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PerfectSpecular {
    kr: Double,
    color: RGBColor
}

// doesn't implement BRDF because we don't use rho or f
impl PerfectSpecular {
    pub fn new(kr: Double, color: RGBColor) -> PerfectSpecular {
        PerfectSpecular {kr, color}
    }

    pub fn sample_f(&self, hit: &Hit, wo: &Vector3D) -> (Vector3D, RGBColor) {
        let ndotwo = hit.normal * *wo;
        let wi = -*wo + 2.0 * hit.normal * ndotwo;
        (wi, (self.color * self.kr) / (hit.normal * wi))
    }
}
