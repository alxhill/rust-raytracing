use crate::render::RenderTarget;
use crate::types::RGBColor;
use crate::world::ViewXY;
use pixel_canvas::{Color, Image, RC, XY};

pub struct CanvasTarget<'a> {
    canvas_img: &'a mut Image,
}

impl CanvasTarget<'_> {
    pub fn new(img: &mut Image) -> CanvasTarget {
        CanvasTarget { canvas_img: img }
    }
}

impl ViewXY {
    fn to_xy(self) -> XY {
        XY(self.x() as usize, self.y() as usize)
    }

    fn to_rc(self) -> RC {
        RC(self.x() as usize, self.y() as usize)
    }
}

impl RenderTarget for CanvasTarget<'_> {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
        let rgb = color.as_rgb();
        self.canvas_img[xy.to_rc()] = Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        };
    }
}
