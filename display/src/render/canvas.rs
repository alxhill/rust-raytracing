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

fn to_xy(xy: &ViewXY) -> XY {
    XY(xy.x() as usize, xy.y() as usize)
}

impl RenderTarget for CanvasTarget<'_> {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
        let rgb = Rgb(color.into());
        self.canvas_img[to_xy(xy)] = Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        };
    }
}
