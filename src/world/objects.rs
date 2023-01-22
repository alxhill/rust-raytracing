use crate::types::{Double, Point3D, RGBColor, Vector3D, BRDF, Shadeable};
use crate::world::tracing::{Hit, Hittable};
use crate::world::Ray;
use std::sync::Arc;

#[derive(Debug)]
pub struct Object {
    pub geometry: Box<dyn Hittable>,
    pub material: Arc<dyn Shadeable>,
}

impl Object {
    pub fn new(geometry: Box<dyn Hittable>, color: RGBColor) -> Object {
        Object {
            geometry,
            material: Arc::new(color)
        }
    }
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.geometry.hit(ray)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    origin: Point3D,
    radius: Double,
}

impl Sphere {
    pub fn new(origin: Point3D, radius: Double) -> Box<Sphere> {
        Box::new(Sphere { origin, radius })
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
            return Some(Hit::hit(
                t,
                ray.origin + (ray.direction * t),
                (ray.origin + (ray.direction * t) - self.origin).normalize(),
            ));
        }

        // larger root
        t = (-b + discriminant.sqrt()) / (2.0 * a);

        if t > Hit::EPSILON {
            return Some(Hit::hit(
                t,
                ray.origin + (ray.direction * t),
                (ray.origin + (ray.direction * t) - self.origin).normalize(),
            ));
        }
        None
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    point: Point3D,
    normal: Vector3D,
}

impl Plane {
    pub fn new(point: Point3D, normal: Vector3D) -> Box<Plane> {
        Box::new(Plane {
            point,
            normal,
        })
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t: Double = (self.point - ray.origin) * self.normal / (ray.direction * self.normal);

        if t > Hit::EPSILON {
            return Some(Hit::hit(
                t,
                ray.origin + (ray.direction * t),
                self.normal
            ));
        }
        None
    }
}
