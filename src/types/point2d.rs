use std::ops::{Add, Mul, Sub};
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

impl Mul<Point2D> for Double {
    type Output = Point2D;

    fn mul(self, rhs: Point2D) -> Self::Output {
        Point2D {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Point2D) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Point2D) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}