use crate::surfaces::brdf::{Glossy, Lambertian, PerfectSpecular, BRDF};
use crate::types::{Double, RGBColor};
use crate::world::{Depth, Hit, Ray, Scene};
use std::fmt::Debug;

pub trait Shadeable: Debug {
    fn shade(&self, hit: Hit, scene: &Scene, depth: Depth) -> RGBColor;
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
            diffuse: Lambertian::new(kd, cd),
        }
    }
}

impl Shadeable for Matte {
    fn shade(&self, hit: Hit, scene: &Scene, _depth: Depth) -> RGBColor {
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
    specular: Glossy,
    reflective: Option<PerfectSpecular>,
}

impl Phong {
    pub fn new(ka: Double, kd: Double, ks: Double, exp: Double, cd: RGBColor) -> Phong {
        Phong {
            ambient: Lambertian::new(ka, cd),
            diffuse: Lambertian::new(kd, cd),
            specular: Glossy::new(ks, exp, cd),
            reflective: None,
        }
    }

    pub fn reflective(
        ka: Double,
        kd: Double,
        ks: Double,
        exp: Double,
        cd: RGBColor,
        kr: Double,
    ) -> Phong {
        Phong {
            ambient: Lambertian::new(ka, cd),
            diffuse: Lambertian::new(kd, cd),
            specular: Glossy::new(ks, exp, cd),
            reflective: Some(PerfectSpecular::new(kr, cd)),
        }
    }
}

impl Shadeable for Phong {
    fn shade(&self, hit: Hit, scene: &Scene, depth: Depth) -> RGBColor {
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
                    l += (self.diffuse.f(&hit, &wi, &wo) + self.specular.f(&hit, &wi, &wo))
                        * light.l()
                        * ndotwi;
                }
            }
        }

        // specular reflection
        if let Some(specular) = self.reflective {
            let (wi, specular_l) = specular.sample_f(&hit, &wo);
            let reflected_ray = Ray::new(hit.hit_loc, wi);

            l += specular_l * scene.render_color(&reflected_ray, depth + 1) * (hit.normal * wi);
        }

        l
    }
}
