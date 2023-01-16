use crate::types::RGBColor;

mod canvas;
mod image;

pub use crate::render::canvas::CanvasRender;
pub use crate::render::image::ImageRender;
use crate::world::ViewXY;

pub trait Renderable {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor);
}
