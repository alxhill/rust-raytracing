#![allow(dead_code)]
extern crate image;
extern crate pixel_canvas;

mod render;
mod types;
mod world;

use crate::render::ImageRender;
use crate::types::*;
use crate::world::*;
use pixel_canvas::input::MouseState;
use pixel_canvas::{Canvas, Color};

fn main() {
    println!("Starting execution.");

    let mut w = World::new();
    w.add(Box::new(Sphere::new(
        Point3D::new(0.0, 0.0, 1.0),
        1.0,
        RGBColor::RED,
    )));
    let mut render = ImageRender::new(128, 128);
    w.render_to(&mut render);

    let flag = std::env::args().nth(1).unwrap_or("--output".to_string());

    match flag.as_str() {
        "--display" => {
            let canvas = Canvas::new(127, 127)
                .title("Tile")
                .hidpi(true)
                .show_ms(true)
                .state(MouseState::new())
                .input(MouseState::handle_input);

            canvas.render(move |mouse, image| {
                let width = image.width() as usize;
                for (y, row) in image.chunks_mut(width).enumerate() {
                    for (x, pixel) in row.iter_mut().enumerate() {
                        let color = render.get_pixel(x as u32, y as u32);
                        *pixel = Color {
                            r: color[0],
                            g: color[1],
                            b: color[2],
                        };
                    }
                }
            });
        }
        "--output" => {
            render.save_image("output.png".to_string());
        }
        _ => println!("Invalid flag: {}", flag),
    }
}
