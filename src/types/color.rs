use crate::types::Double;
use num_traits::NumCast;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, BitXor, Div, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RGBColor {
    pub r: Double,
    pub g: Double,
    pub b: Double,
}

impl Display for RGBColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGBColor({:.2}, {:.2}, {:.2})", self.r, self.g, self.b)
    }
}

impl RGBColor {
    pub const fn new(r: Double, g: Double, b: Double) -> RGBColor {
        RGBColor { r, g, b }
    }

    pub const WHITE: RGBColor = RGBColor::new(1.0, 1.0, 1.0);
    pub const RED: RGBColor = RGBColor::new(1.0, 0.0, 0.0);
    pub const GREEN: RGBColor = RGBColor::new(0.0, 1.0, 0.0);
    pub const YELLOW: RGBColor = RGBColor::new(1.0, 1.0, 0.0);
    pub const GREY: RGBColor = RGBColor::new(0.5, 0.5, 0.5);
    pub const BLACK: RGBColor = RGBColor::new(0.0, 0.0, 0.0);
}

impl From<[u8; 3]> for RGBColor {
    fn from([r, g, b]: [u8; 3]) -> Self {
        RGBColor {
            r: r as Double / 255.0,
            g: g as Double / 255.0,
            b: b as Double / 255.0,
        }
    }
}

impl From<&RGBColor> for [u8; 3] {
    fn from(color: &RGBColor) -> Self {
        [
            (color.r.clamp(0.0, 1.0) * 255.0) as u8,
            (color.g.clamp(0.0, 1.0) * 255.0) as u8,
            (color.b.clamp(0.0, 1.0) * 255.0) as u8,
        ]
    }
}

impl BitXor<Double> for RGBColor {
    type Output = RGBColor;

    fn bitxor(self, rhs: Double) -> Self::Output {
        RGBColor::new(self.r.powf(rhs), self.g.powf(rhs), self.b.powf(rhs))
    }
}

impl Add for RGBColor {
    type Output = RGBColor;

    fn add(self, rhs: Self) -> Self::Output {
        RGBColor::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for RGBColor {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl<T: NumCast> Div<T> for RGBColor {
    type Output = RGBColor;

    fn div(self, rhs: T) -> Self::Output {
        let val: Double = num_traits::cast(rhs).unwrap();
        RGBColor::new(self.r / val, self.g / val, self.b / val)
    }
}

impl<T: NumCast> Mul<T> for RGBColor {
    type Output = RGBColor;

    fn mul(self, rhs: T) -> Self::Output {
        let val: Double = num_traits::cast(rhs).unwrap();
        RGBColor::new(self.r * val, self.g * val, self.b * val)
    }
}

impl Mul for RGBColor {
    type Output = RGBColor;

    fn mul(self, rhs: Self) -> Self::Output {
        RGBColor::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}
