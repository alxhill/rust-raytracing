use crate::render::Renderable;
use crate::types::RGBColor;
use pixel_canvas::{Color, Image, RC, XY};

pub struct CanvasRender<'a> {
    canvas_img: &'a mut Image,
}

impl CanvasRender<'_> {
    pub fn new(img: &mut Image) -> CanvasRender {
        CanvasRender { canvas_img: img }
    }
}

impl Renderable for CanvasRender<'_> {
    fn set_pixel(&mut self, x: u32, y: u32, color: RGBColor) {
        let rgb = color.to_rgb();
        println!(
            "Setting pixel at ({}, {}) to ({}, {}, {})",
            x, y, rgb[0], rgb[1], rgb[2]
        );
        self.canvas_img[XY(x as usize, y as usize)] = Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        };
    }
}
