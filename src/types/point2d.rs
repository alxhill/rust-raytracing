use crate::types;
use crate::types::{Distance, Double};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point2D {
    x: Double,
    y: Double,
}

impl Point2D {
    pub fn zero() -> Point2D {
        Point2D {x: 0.0, y: 0.0}
    }
    pub fn both(val: Double) -> Point2D {
        Point2D { x: val, y: val }
    }
    pub fn new(x: Double, y: Double) -> Point2D {
        Point2D {x, y}
    }
}

impl Distance for Point2D {
    fn dist(&self, other: Point2D) -> Double {
        (types::diff_sq(self.x, other.x) + types::diff_sq(self.y, other.y)).sqrt()
    }
}
