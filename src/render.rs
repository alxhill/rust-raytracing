use crate::types::RGBColor;

mod image;
mod canvas;

pub use crate::render::image::ImageRender;
pub use crate::render::canvas::CanvasRender;

pub trait Renderable {
    fn set_pixel(&mut self, x: u32, y: u32, color: RGBColor);
}
