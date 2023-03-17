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
    ABox(ABox),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray),
            Geometry::Plane(plane) => plane.hit(ray),
            Geometry::ABox(a_box) => a_box.hit(ray),
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

    pub fn a_box(a_box: ABox, material: Arc<dyn Shadeable>) -> Object {
        Object::new(Geometry::ABox(a_box), material)
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
pub struct ABox {
    min: Point3D,
    max: Point3D,
}

impl ABox {
    #[inline(always)]
    pub fn new(min: Point3D, max: Point3D) -> ABox {
        ABox { min, max }
    }
}

impl Hittable for ABox {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let mut t_min: Double = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut t_max: Double = (self.max.x - ray.origin.x) / ray.direction.x;

        if t_min > t_max {
            std::mem::swap(&mut t_min, &mut t_max);
        }

        let mut t_y_min: Double = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut t_y_max: Double = (self.max.y - ray.origin.y) / ray.direction.y;

        if t_y_min > t_y_max {
            std::mem::swap(&mut t_y_min, &mut t_y_max);
        }

        if t_min > t_y_max || t_y_min > t_max {
            return None;
        }

        if t_y_min > t_min {
            t_min = t_y_min;
        }

        if t_y_max < t_max {
            t_max = t_y_max;
        }

        let mut t_z_min: Double = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut t_z_max: Double = (self.max.z - ray.origin.z) / ray.direction.z;

        if t_z_min > t_z_max {
            std::mem::swap(&mut t_z_min, &mut t_z_max);
        }

        if t_min > t_z_max || t_z_min > t_max {
            return None;
        }

        if t_z_min > t_min {
            t_min = t_z_min;
        }

        if t_z_max < t_max {
            t_max = t_z_max;
        }

        let hit_loc = ray.at(t_min);
        let normal = if t_min == t_z_min {
            Vector3D::new(0.0, 0.0, -1.0)
        } else if t_min == t_z_max {
            Vector3D::new(0.0, 0.0, 1.0)
        } else if t_min == t_y_min {
            Vector3D::new(0.0, -1.0, 0.0)
        } else if t_min == t_y_max {
            Vector3D::new(0.0, 1.0, 0.0)
        } else if t_min == t_min {
            Vector3D::new(-1.0, 0.0, 0.0)
        } else {
            Vector3D::new(1.0, 0.0, 0.0)
        };

        Some(Hit::new(t_min, hit_loc, *ray, normal))
    }
}