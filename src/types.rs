mod color;
mod material;
mod matrix;
mod point2d;
mod point3d;
mod utils;
mod vector3d;

pub use color::*;
pub use material::*;
pub use matrix::Matrix;
pub use point2d::Point2D;
pub use point3d::Point3D;
pub use vector3d::Vector3D;

pub type Float = f32;
pub type Double = f64;

pub trait Clamp {
    fn clamp(&self) -> Double;
}
