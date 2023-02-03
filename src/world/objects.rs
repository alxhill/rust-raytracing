use crate::types::{Double, Point3D, Shadeable, Vector3D};
use crate::world::tracing::{Hit, Hittable};
use crate::world::Ray;
use std::sync::Arc;

#[derive(Debug)]
pub struct Object<'a> {
    pub geometry: bumpalo::boxed::Box<dyn Hittable<'a>>,
    pub material: Arc<dyn Shadeable>,
}

impl Object<'_> {
    pub fn new(geometry: Box<dyn Hittable>, material: Arc<dyn Shadeable>) -> Object {
        Object {
            geometry,
            material,
        }
    }
}

impl Hittable<'_> for Object<'_> {
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
    #[inline(always)]
    pub fn new(origin: Point3D, radius: Double) -> Sphere {
        Sphere { origin, radius }
    }
}

impl Hittable<'_> for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc: Vector3D = ray.origin - self.origin;
        let a: Double = ray.direction * ray.direction;
        let b: Double = 2.0 * (oc * ray.direction);
        let c: Double = oc * oc - self.radius * self.radius;
        let discriminant: Double = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let small_root = (-b - discriminant.sqrt()) / (2.0 * a);
        let large_root = (-b + discriminant.sqrt()) / (2.0 * a);

        if small_root > Hit::EPSILON || large_root > Hit::EPSILON {
            let t = small_root.min(large_root);
            let hit_loc = ray.origin + t * ray.direction;
            let normal = (hit_loc - self.origin).normalize();
            return Some(Hit::new(t, hit_loc, *ray, normal));
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
        Box::new(Plane { point, normal })
    }
}

impl Hittable<'_> for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t: Double = (self.point - ray.origin) * self.normal / (ray.direction * self.normal);

        if t > Hit::EPSILON {
            return Some(Hit::new(t, ray.origin + (ray.direction * t), *ray, self.normal));
        }
        None
    }
}
