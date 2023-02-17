use image::Rgb;
use pixel_canvas::{Color, Image, XY};
use rust_raytracing::render::RenderTarget;
use rust_raytracing::{RGBColor, ViewXY};

pub struct CanvasTarget<'a> {
    canvas_img: &'a mut Image,
}

impl CanvasTarget<'_> {
    pub fn new(img: &mut Image) -> CanvasTarget {
        CanvasTarget { canvas_img: img }
    }
}

fn to_xy(xy: &ViewXY, height: usize) -> XY {
    XY(xy.x() as usize, height - xy.y() as usize - 1)
}

impl RenderTarget for CanvasTarget<'_> {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
        let rgb = Rgb(color.into());
        let h = self.canvas_img.height();
        self.canvas_img[to_xy(xy, h)] = Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        };
    }
}
