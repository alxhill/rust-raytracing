use crate::types::{Double, Point3D, Vector3D};
use std::fmt::Debug;
use std::sync::Arc;
use crate::world::Object;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }
}

#[derive(Clone, Debug)]
pub struct Hit {
    pub t: Double,
    pub hit_loc: Point3D,
    pub normal: Vector3D,
    pub ray: Ray,
    // todo: deprecate Object and find cleaner way
    // to pass around material info
    pub object: Option<Arc<Object>>,
    pub depth: u8,
}

impl Hit {
    pub fn new(dist: Double, hit_loc: Point3D, ray: Ray, normal: Vector3D) -> Hit {
        Hit {
            t: dist,
            hit_loc,
            normal,
            ray,
            object: None,
            depth: 0,
        }
    }

    pub fn set_obj(&mut self, object: Arc<Object>) {
        self.object = Some(object)
    }

    pub const EPSILON: Double = 1e-6;
}

pub trait Hittable: Debug {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
