use crate::types::Float;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RGBColor {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl RGBColor {
    pub const fn new(r: Float, g: Float, b: Float) -> RGBColor {
        RGBColor { r, g, b }
    }

    pub fn to_u8(&self) -> [u8; 3] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ]
    }

    pub const WHITE: RGBColor = RGBColor::new(1.0, 1.0, 1.0);
    pub const RED: RGBColor = RGBColor::new(1.0, 0.0, 0.0);
    pub const GREEN: RGBColor = RGBColor::new(0.0, 1.0, 0.0);
    pub const BLACK: RGBColor = RGBColor::new(0.0, 0.0, 0.0);
}
