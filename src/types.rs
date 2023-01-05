mod point2d;
mod point3d;
mod matrix;
mod ray;
mod vector3d;

pub use point2d::Point2D;
pub use point3d::Point3D;
pub use matrix::Matrix;

pub type Float = f32;
pub type Double = f64;

trait Distance<Other=Self> {
    fn dist(&self, other: Other) -> Double;
}

fn diff_sq(first: Double, second: Double) -> Double {
    (first - second).powi(2)
}
