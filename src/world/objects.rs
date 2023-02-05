use crate::types::{Double, Point3D, Shadeable, Vector3D};
use crate::world::tracing::{Hit, Hittable};
use crate::world::Ray;

#[derive(Debug)]
pub struct Object<'w> {
    pub geometry: Geometry,
    pub material: &'w dyn Shadeable,
}

#[derive(Debug, Copy, Clone)]
pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane)
}

impl<'t> Hittable<'t> for Geometry {
    fn hit(&self, ray: &Ray) -> Option<Hit<'t>> {
        match self {
            Geometry::Sphere(sphere) => {
                sphere.hit(ray)
            }
            Geometry::Plane(plane) => {
                plane.hit(ray)
            }
        }
    }
}

impl<'w> Object<'w> {
    pub fn new(geometry: Geometry, material: &'w dyn Shadeable) -> Object<'w> {
        Object {
            geometry,
            material,
        }
    }

    pub fn sphere(sphere: Sphere, material: &'w dyn Shadeable) -> Object<'w> {
        Object::new(Geometry::Sphere(sphere), material)
    }

    pub fn plane(plane: Plane, material: &'w dyn Shadeable) -> Object<'w> {
        Object::new(Geometry::Plane(plane), material)
    }
}

impl<'t> Hittable<'t> for Object<'t> {
    fn hit(&self, ray: &Ray) -> Option<Hit<'t>> {
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

impl<'t> Hittable<'t> for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit<'t>> {
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
    #[inline(always)]
    pub fn new(point: Point3D, normal: Vector3D) -> Plane {
        Plane { point, normal }
    }
}

impl<'t> Hittable<'t> for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit<'t>> {
        let t: Double = (self.point - ray.origin) * self.normal / (ray.direction * self.normal);

        if t > Hit::EPSILON {
            return Some(Hit::new(t, ray.origin + (ray.direction * t), *ray, self.normal));
        }
        None
    }
}
