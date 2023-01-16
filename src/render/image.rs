use crate::render::Renderable;
use crate::types::{Double, Float, RGBColor};
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

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 3] {
        self.buffer.get_pixel(x, y).0
    }
}

impl Renderable for ImageRender {
    fn set_pixel(&mut self, x: u32, y: u32, color: RGBColor) {
        self.buffer.put_pixel(x, y, color.to_rgb());
    }
}
