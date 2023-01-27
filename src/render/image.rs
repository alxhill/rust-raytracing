use crate::render::RenderTarget;
use crate::types::RGBColor;
use crate::world::ViewXY;
use image::{ImageBuffer, Rgb};

pub struct ImageTarget {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl ImageTarget {
    pub fn new(width: u32, height: u32) -> ImageTarget {
        ImageTarget {
            buffer: ImageBuffer::new(width, height),
        }
    }

    pub fn save_image(&self, name: String) {
        self.buffer.save(name).expect("Failed to save image");
    }
}

impl RenderTarget for ImageTarget {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
        self.buffer.put_pixel(xy.x(), xy.y(), color.as_rgb());
    }
}
