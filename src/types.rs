mod point2d;
mod point3d;
mod matrix;
mod ray;
mod vector3d;
mod color;
mod normal;
mod utils;

pub use point2d::Point2D;
pub use point3d::Point3D;
pub use matrix::Matrix;
pub use ray::Ray;
pub use vector3d::Vector3D;
pub use color::RGBColor;
pub use normal::Normal;

pub type Float = f32;
pub type Double = f64;

trait Distance<Other=Self> {
    fn dist(&self, other: &Other) -> Double;
}
