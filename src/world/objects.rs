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
    AABox(AABox),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray),
            Geometry::Plane(plane) => plane.hit(ray),
            Geometry::AABox(a_box) => a_box.hit(ray),
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

    pub fn aa_box(aa_box: AABox, material: Arc<dyn Shadeable>) -> Object {
        Object::new(Geometry::AABox(aa_box), material)
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

        let small_root = (-b - discriminant.sqrt()) / (2.0 * a);
        let large_root = (-b + discriminant.sqrt()) / (2.0 * a);

        if small_root > Hit::EPSILON || large_root > Hit::EPSILON {
            let t = small_root.min(large_root);
            let hit_loc = ray.at(t);
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

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t: Double = (self.point - ray.origin) * self.normal / (ray.direction * self.normal);

        if t > Hit::EPSILON {
            return Some(Hit::new(t, ray.at(t), *ray, self.normal));
        }
        None
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AABox {
    min: Point3D,
    max: Point3D,
}

impl AABox {
    #[inline(always)]
    pub fn new(min: Point3D, max: Point3D) -> AABox {
        AABox { min, max }
    }
}

impl Hittable for AABox {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let mut t_min: Double = 0.00001; // epsilon value
        let mut t_max: Double = f64::INFINITY;
        let mut face_axis = 0;
        let mut face_is_min = false;

        for axis in 0..3 {
            let inv_d = 1.0 / ray.direction[axis];
            let mut t0 = (self.min[axis] - ray.origin[axis]) * inv_d;
            let mut t1 = (self.max[axis] - ray.origin[axis]) * inv_d;
            let is_min_axis = inv_d < 0.0;

            if is_min_axis {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > t_min {
                t_min = t0;
                face_axis = axis;
                face_is_min = is_min_axis;
            }

            t_max = t1.min(t_max);

            if t_max <= t_min {
                return None;
            }
        }

        let hit_loc = ray.at(t_min);
        let mut normal = Vector3D { x: 0.0, y: 0.0, z: 0.0 };
        normal[face_axis] = if face_is_min { 1.0 } else { -1.0 };

        let hit = Hit::new(t_min, hit_loc, ray.clone(), normal);
        Some(hit)
    }
}