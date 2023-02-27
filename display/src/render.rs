use rayon::prelude::*;
use rust_raytracing::render::Renderable;
use rust_raytracing::{RGBColor, RenderTarget, ViewPlane, ViewXY};

pub mod canvas;
pub mod image;

pub fn render_parallel<R: Renderable + Sync, T: RenderTarget>(
    view_plane: &ViewPlane,
    renderer: &R,
    img: &mut T,
) {
    let pixels: Vec<(ViewXY, RGBColor)> = ParallelIterator::map(view_plane.into_par_iter(), |xy| {
        (xy, renderer.render_pixel(&xy))
    })
    .collect();
    for pixel in pixels {
        img.set_pixel(&pixel.0, &pixel.1);
    }
}
