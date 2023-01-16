#![allow(dead_code)]
extern crate image;
extern crate pixel_canvas;

mod render;
mod types;
mod world;

use crate::render::{CanvasRender, ImageRender};
use crate::types::*;
use crate::world::*;
use pixel_canvas::input::MouseState;
use pixel_canvas::Canvas;

fn main() {
    println!("Starting execution.");

    let mut w = World::new();
    w.add(Box::new(Sphere::new(Point3D::zero(), 50.0, RGBColor::RED)));

    let flag = std::env::args().nth(1).unwrap_or("--output".to_string());

    match flag.as_str() {
        "--display" => {
            let canvas = Canvas::new(128, 128)
                .title("Tile")
                .state(MouseState::new())
                .input(MouseState::handle_input);

            let mut render_count: u32 = 0;
            println!("Starting display.");
            canvas.render(move |_, image| {
                print!(".");
                render_count += 1;
                if render_count % 100 == 0 {
                    print!("\n");
                }
                let mut canvas_render = CanvasRender::new(image);
                w.render_to(&mut canvas_render);
            });
        }
        "--output" => {
            let mut render = ImageRender::new(128, 128);
            w.render_to(&mut render);
            render.save_image("output.png".to_string());
        }
        _ => println!("Invalid flag: {}", flag),
    }
}
