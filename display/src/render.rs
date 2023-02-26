use rayon::prelude::*;
use rust_raytracing::render::Renderable;
use rust_raytracing::{RenderTarget, ViewPlane, ViewXY};

pub mod canvas;
pub mod image;

pub fn render_parallel<R: Renderable + Send + Sync, T: RenderTarget + Send + Sync>(
    view_plane: &mut ViewPlane,
    mut renderer: R,
    mut img: T,
) {
    // ParallelIterator::for_each(view_plane.into_par_iter(), move |xy| {
    //     img.set_pixel(&xy, &renderer.render_pixel(&xy));
    // });
}
