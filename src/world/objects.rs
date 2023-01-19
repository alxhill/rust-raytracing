use crate::types::{Double, Lambertian, Point3D, RGBColor, Vector3D, BRDF};
use crate::world::tracing::{Hit, Hittable};
use crate::world::Ray;
use std::sync::Arc;

#[derive(Debug)]
pub struct Object {
    pub geometry: Box<dyn Hittable>,
    pub material: Arc<dyn BRDF>,
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    origin: Point3D,
    radius: Double,
}

impl Sphere {
    pub fn new(origin: Point3D, radius: Double, mat: Arc<dyn BRDF>) -> Object {
        let sphere = Sphere { origin, radius };
        Object {
            geometry: Box::new(sphere),
            material: mat,
        }
    }

    // todo: move this somewhere better
    fn get_hit(&self, ray: &Ray, t: Double) -> Hit {
        Hit::hit(
            t,
            ray.origin + (ray.direction * t),
            (ray.origin + (ray.direction * t) - self.origin).normalize(),
            self.color,
        )
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc: Vector3D = ray.origin - self.origin;
        let a: Double = ray.direction * ray.direction;
        let b: Double = 2.0 * (oc * ray.direction);
        let c: Double = oc * oc - self.radius * self.radius;
        let discriminant: Double = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        // smaller root
        let mut t: Double = (-b - discriminant.sqrt()) / (2.0 * a);

        if t > Hit::EPSILON {
            return Some(self.get_hit(ray, t));
        }

        // larger root
        t = (-b + discriminant.sqrt()) / (2.0 * a);

        if t > Hit::EPSILON {
            return Some(self.get_hit(ray, t));
        }
        None
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    point: Point3D,
    normal: Vector3D,
    color: RGBColor, // temporary
}

impl Plane {
    pub fn new(point: Point3D, normal: Vector3D, color: RGBColor) -> Plane {
        Plane {
            point,
            normal,
            color,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t: Double = (self.point - ray.origin) * self.normal / (ray.direction * self.normal);

        if t > Hit::EPSILON {
            return Some(Hit::hit(
                t,
                ray.origin + (ray.direction * t),
                self.normal,
                self.color,
            ));
        }
        None
    }
}
