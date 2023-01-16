use crate::types::RGBColor;

mod image;

pub use crate::render::image::ImageRender;

pub trait Renderable {
    fn set_pixel(&mut self, x: u32, y: u32, color: RGBColor);
}
