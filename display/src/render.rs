use rayon::prelude::*;
use rust_raytracing::render::Renderable;
use rust_raytracing::{RGBColor, RenderTarget, ViewPlane, ViewXY};

pub mod canvas;
pub mod image;

const CHUNK_SIZE: usize = 32768;

pub fn render_parallel<R: Renderable + Sync, T: RenderTarget>(
    view_plane: &ViewPlane,
    renderer: &R,
    img: &mut T,
) {
    let pixel_array = view_plane.pixel_array();

    let output = pixel_array
        .par_chunks(CHUNK_SIZE)
        .map(|chunk| {
            let mut output_row = [RGBColor::BLACK; CHUNK_SIZE];
            for (i, xy) in (*chunk).iter().enumerate() {
                output_row[i] = renderer.render_pixel(xy);
            }
            output_row
        })
        .flatten()
        .collect::<Vec<RGBColor>>();

    for (i, color) in output.iter().enumerate() {
        img.set_pixel(&ViewXY(i / view_plane.width, i % view_plane.height), color);
    }
}
