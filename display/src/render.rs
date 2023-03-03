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
    let pixel_array = view_plane.pixel_array();
    let pixel_chunks = pixel_array.chunks(1024).collect::<Vec<_>>();

    let output = pixel_chunks.par_iter().map(|chunk| {
        let mut output_row = Vec::new();
        for xy in *chunk {
            output_row.push(renderer.render_pixel(xy));
        }
        output_row
    }).flatten().collect::<Vec<RGBColor>>();

    for (i, color) in output.iter().enumerate() {
        img.set_pixel(&ViewXY(i % view_plane.width, i / view_plane.height), color);
    }
}
