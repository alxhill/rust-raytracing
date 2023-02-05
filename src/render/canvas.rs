use crate::render::RenderTarget;
use crate::types::RGBColor;
use crate::world::ViewXY;
use pixel_canvas::{Color, Image, XY};

pub struct CanvasTarget<'a> {
    canvas_img: &'a mut Image,
}

impl CanvasTarget<'_> {
    pub fn new(img: &mut Image) -> CanvasTarget {
        CanvasTarget { canvas_img: img }
    }
}

impl ViewXY {
    fn to_xy(self, height: usize) -> XY {
        XY(self.x() as usize, height - self.y() as usize - 1)
    }
}

impl RenderTarget for CanvasTarget<'_> {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
        let rgb = color.as_rgb();
        let h = self.canvas_img.height();
        self.canvas_img[xy.to_xy(h)] = Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        };
    }
}
