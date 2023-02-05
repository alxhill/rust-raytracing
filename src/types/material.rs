use crate::types::{Double, RGBColor, Vector3D};
use crate::world::{Hit, Ray, Scene};
use std::fmt::Debug;
use num_traits::Pow;

pub trait Shadeable: Debug {
    fn shade(&self, hit: Hit, scene: &Scene) -> RGBColor;
}

pub trait BRDF: Debug {
    fn f(&self, hit: &Hit, wi: &Vector3D, wo: &Vector3D) -> RGBColor;
    fn rho(&self, hit: &Hit, wo: &Vector3D) -> RGBColor;
}

#[derive(Debug, Copy, Clone)]
pub struct Matte {
    ambient: Lambertian,
    diffuse: Lambertian,
}

impl Matte {
    #[inline(always)]
    pub fn new(ka: Double, kd: Double, cd: RGBColor) -> Matte {
        Matte {
            ambient: Lambertian::new(ka, cd),
            diffuse: Lambertian::new(kd, cd)
        }
    }
}

impl Shadeable for Matte {
    fn shade(&self, hit: Hit, scene: &Scene) -> RGBColor {
        let wo = -hit.ray.direction;
        let mut l = self.ambient.rho(&hit, &wo);
        for light in scene.lights.iter() {
            let wi = light.direction(&hit.hit_loc);
            let ndotwi = hit.normal * wi;
            if ndotwi > 0.0 {
                l += self.diffuse.f(&hit, &wo, &wi) * light.l() * ndotwi;
            }
        }
        l
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Phong {
    ambient: Lambertian,
    diffuse: Lambertian,
    specular: Glossy
}

impl Phong {
    pub fn new(ka: Double, kd: Double, ks: Double, exp: Double, cd: RGBColor) -> Phong {
        Phong {
            ambient: Lambertian::new(ka, cd),
            diffuse: Lambertian::new(kd, cd),
            specular: Glossy::new(ks, exp, cd)
        }
    }
}

impl Shadeable for Phong {
    fn shade(&self, hit: Hit, scene: &Scene) -> RGBColor {
        let wo = -hit.ray.direction;
        let mut l = self.ambient.rho(&hit, &wo);
        for light in scene.lights.iter() {
            let wi = light.direction(&hit.hit_loc);
            let ndotwi = hit.normal * wi;
            if ndotwi > 0.0 {
                let mut in_shadow = false;

                if light.casts_shadows {
                    let shadow_ray = Ray::new(hit.hit_loc, wi);
                    in_shadow = light.in_shadow(shadow_ray, scene);
                }

                if !in_shadow {
                    l += (self.diffuse.f(&hit, &wi, &wo)
                        + self.specular.f(&hit, &wi, &wo)) * light.l() * ndotwi;
                }
            }
        }

        l
    }
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