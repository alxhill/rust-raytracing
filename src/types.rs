mod color;
mod matrix;
mod point2d;
mod point3d;
mod utils;
mod vector3d;

pub use crate::world::ray::Ray;
pub use color::*;
pub use matrix::Matrix;
pub use point2d::Point2D;
pub use point3d::Point3D;
pub use vector3d::Vector3D;

pub type Float = f32;
pub type Double = f64;

trait Distance<Other = Self> {
    fn dist(&self, other: &Other) -> Double;
}
