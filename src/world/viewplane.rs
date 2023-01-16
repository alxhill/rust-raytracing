use crate::types::Double;

type X = u32;
type Y = u32;

#[derive(Debug, Copy, Clone)]
pub struct ViewXY(pub X, pub Y);

impl ViewXY {
    pub fn x(&self) -> X {
        self.0
    }
    pub fn y(&self) -> Y {
        self.1
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ViewPlane {
    pub width: X,
    pub height: Y,
    pub pixel_size: Double,
    pub gamma: Double,
    pub inv_gamma: Double,
    pub show_out_of_gamut: bool,
}

impl ViewPlane {
    pub fn new(width: u32, height: u32, pixel_size: Double) -> ViewPlane {
        ViewPlane {
            width,
            height,
            pixel_size,
            gamma: 1.0,
            inv_gamma: 1.0,
            show_out_of_gamut: false,
        }
    }

    pub fn for_each_pixel(&self, mut f: impl FnMut(&ViewXY)) {
        for x in 0..self.width {
            for y in 0..self.height {
                f(&ViewXY(x, y));
            }
        }
    }
}
