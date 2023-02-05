use crate::types::{Double, Point3D, RGBColor, Vector3D};
use crate::world::{Hit, Hittable, Ray, Scene};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    color: RGBColor,
    location: Point3D,
    pub casts_shadows: bool,
    ls: Double // may become a material in future
}

impl Light {
    pub fn point_light(point: Point3D, ls: Double) -> Light {
        Light {
            color: RGBColor::WHITE,
            location: point,
            casts_shadows: true,
            ls
        }
    }

    pub fn direction(&self, hit_point: &Point3D) -> Vector3D {
        (self.location - *hit_point).normalize()
    }

    pub fn l(&self) -> RGBColor {
        self.color * self.ls
    }

    pub fn in_shadow(&self, shadow_ray: Ray, scene: &Scene) -> bool {
        let dist = (self.location - shadow_ray.origin).magnitude();
        for object in scene.objects.iter() {
            if let Some(shadow_hit) = object.hit(&shadow_ray) {
                if shadow_hit.t < dist {
                    return true
                }
            }
        }

        false
    }
}