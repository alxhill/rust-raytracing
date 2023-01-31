use crate::types::Double;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point2D {
    pub x: Double,
    pub y: Double,
}

impl Point2D {
    pub fn zero() -> Point2D {
        Point2D { x: 0.0, y: 0.0 }
    }
    pub fn both(val: Double) -> Point2D {
        Point2D { x: val, y: val }
    }
    pub fn new(x: Double, y: Double) -> Point2D {
        Point2D { x, y }
    }
}
