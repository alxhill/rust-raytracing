use crate::surfaces::Shadeable;
use crate::types::{Double, Point3D, Vector3D};
use crate::world::tracing::{Hit, Hittable};
use crate::world::Ray;
use std::sync::Arc;

#[derive(Debug)]
pub struct Object {
    pub geometry: Geometry,
    pub material: Arc<dyn Shadeable>,
}

#[derive(Debug, Copy, Clone)]
pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray),
            Geometry::Plane(plane) => plane.hit(ray),
        }
    }
}

impl Object {
    pub fn new(geometry: Geometry, material: Arc<dyn Shadeable>) -> Object {
        Object { geometry, material }
    }

    pub fn sphere(sphere: Sphere, material: Arc<dyn Shadeable>) -> Object {
        Object::new(Geometry::Sphere(sphere), material)
    }

    pub fn plane(plane: Plane, material: Arc<dyn Shadeable>) -> Object {
        Object::new(Geometry::Plane(plane), material)
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
    #[inline(always)]
    pub fn new(origin: Point3D, radius: Double) -> Sphere {
        Sphere { origin, radius }
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

        let mut tmin: Option<Double> = None;

        let small_root = (-b - discriminant.sqrt()) / (2.0 * a);
        if small_root > Hit::EPSILON {
            tmin = Some(small_root);
        }

        let large_root = (-b + discriminant.sqrt()) / (2.0 * a);

        if tmin.is_none() && large_root > Hit::EPSILON {
            tmin = Some(large_root);
        }

        if let Some(t) = tmin {
            let hit_loc = ray.at(t);
            let normal = (hit_loc - self.origin) / self.radius;
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
    #[inline(always)]
    pub fn new(point: Point3D, normal: Vector3D) -> Plane {
        Plane { point, normal }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t: Double = (self.point - ray.origin) * self.normal / (ray.direction * self.normal);

        if t > Hit::EPSILON {
            return Some(Hit::new(t, ray.at(t), *ray, self.normal));
        }
        None
    }
}
