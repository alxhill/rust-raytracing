use crate::types::{Double, Point3D, RGBColor, Vector3D};
use crate::world::tracing::{Hit, Hittable};
use crate::world::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    point: Point3D,
    normal: Vector3D,
    color: RGBColor, // temporary
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    origin: Point3D,
    radius: Double,
    color: RGBColor, // temporary
}

impl Sphere {
    pub fn new(origin: Point3D, radius: Double, color: RGBColor) -> Sphere {
        Sphere {
            origin,
            radius,
            color,
        }
    }

    fn get_hit(&self, ray: &Ray, t: &Double) -> Hit {
        Hit::hit(
            ray.origin + (ray.direction * *t),
            (ray.origin + (ray.direction * *t) - self.origin).normalize(),
            self.color,
        )
    }
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
    fn hit(&self, ray: &Ray, tmin: &mut Double) -> Option<Hit> {
        let v: Vector3D = self.point - ray.origin;
        let t: Double = ((self.point - ray.origin) * self.normal) / (ray.direction * self.normal);

        if t > Hit::EPSILON {
            *tmin = t;
            return Some(Hit::hit(
                ray.origin + (ray.direction * t),
                self.normal,
                self.color,
            ));
        }
        None
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: &mut Double) -> Option<Hit> {
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
            *tmin = t;
            return Some(self.get_hit(ray, &t));
        }

        // larger root
        t = (-b + discriminant.sqrt()) / (2.0 * a);

        if t > Hit::EPSILON {
            *tmin = t;
            return Some(self.get_hit(ray, &t));
        }
        None
    }
}
