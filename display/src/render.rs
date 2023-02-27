use rayon::prelude::*;
use rust_raytracing::render::Renderable;
use rust_raytracing::{RenderTarget, ViewPlane, ViewXY};

pub mod canvas;
pub mod image;

pub fn render_parallel<R: Renderable + Sync, T: RenderTarget + Sync>(
    view_plane: &ViewPlane,
    renderer: &R,
    img: &T,
) {
    ParallelIterator::for_each(view_plane.into_par_iter(), |xy| {
        let pixel = renderer.render_pixel(&xy);
        img.set_pixel_par(&xy, &pixel);
    });
}
