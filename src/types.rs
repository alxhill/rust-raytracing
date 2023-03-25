mod color;
mod matrix;
mod point2d;
mod point3d;
mod vector3d;

pub use color::*;
pub use matrix::Matrix;
pub use point2d::Point2D;
pub use point3d::Point3D;
pub use vector3d::Vector3D;

pub type Double = f64;

pub trait Axis {
    fn new(x: Double, y: Double, z: Double) -> Self;
    fn zero() -> Self;
    fn x(&self) -> Double;
    fn y(&self) -> Double;
    fn z(&self) -> Double;
}