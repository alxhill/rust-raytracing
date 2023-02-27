use image::{ImageBuffer, Rgb};
use rust_raytracing::render::RenderTarget;
use rust_raytracing::{RGBColor, ViewXY};

pub type RtImageBuffer = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub struct ImageTarget {
    pub buffer: RtImageBuffer,
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
        self.buffer
            .put_pixel(xy.x(), self.buffer.height() - xy.y() - 1, Rgb(color.into()));
    }

    fn set_pixel_par(&self, xy: &ViewXY, color: &RGBColor) {
        self.buffer
            .put_pixel(xy.x(), self.buffer.height() - xy.y() - 1, Rgb(color.into()));
    }
}

pub fn copy_to<T: RenderTarget>(image: &RtImageBuffer, target: &mut T) {
    for x in 0..image.width() {
        for y in 0..image.height() {
            target.set_pixel(&ViewXY(x, y), &RGBColor::from(image.get_pixel(x, y).0))
        }
    }
}
