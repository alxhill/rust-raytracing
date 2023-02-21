use crate::types::Double;
use num_traits::NumCast;
use std::ops::{Add, Div, Mul, Sub};

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

    pub fn swap(&self) -> Point2D {
        Point2D {
            x: self.y,
            y: self.x,
        }
    }
}

impl<T: NumCast> Mul<T> for Point2D {
    type Output = Point2D;

    fn mul(self, rhs: T) -> Self::Output {
        let val: Double = num_traits::cast(rhs).unwrap();
        Point2D {
            x: self.x * val,
            y: self.y * val,
        }
    }
}

impl<T: NumCast> Add<T> for Point2D {
    type Output = Point2D;

    fn add(self, rhs: T) -> Self::Output {
        let val: Double = num_traits::cast(rhs).unwrap();
        Point2D {
            x: self.x + val,
            y: self.y + val,
        }
    }
}

impl<T: NumCast> Div<T> for Point2D {
    type Output = Point2D;

    fn div(self, rhs: T) -> Self::Output {
        let val: Double = num_traits::cast(rhs).unwrap();
        Point2D {
            x: self.x / val,
            y: self.y / val,
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
