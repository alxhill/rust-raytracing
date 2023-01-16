use crate::render::Renderable;
use crate::types::RGBColor;
use crate::world::ViewXY;
use image::{ImageBuffer, Rgb};

pub struct ImageRender {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl ImageRender {
    pub fn new(width: u32, height: u32) -> ImageRender {
        ImageRender {
            buffer: ImageBuffer::new(width, height),
        }
    }

    pub fn save_image(&self, name: String) {
        self.buffer.save(name).expect("Failed to save image");
    }
}

impl Renderable for ImageRender {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
        self.buffer.put_pixel(xy.x(), xy.y(), color.to_rgb());
    }
}
