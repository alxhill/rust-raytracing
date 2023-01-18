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
use std::io::Write;

fn main() {
    println!("Starting execution.");

    let mut s = Scene::new();

    s.add(Box::new(Sphere::new(Point3D::zero(), 40.0, RGBColor::RED)));
    s.add(Box::new(Sphere::new(
        Point3D::new(0.0, 20.0, -1.0),
        30.0,
        RGBColor::YELLOW,
    )));
    s.add(Box::new(Plane::new(
        Point3D::zero(),
        Vector3D::new(0.0, 1.0, 1.0),
        RGBColor::BLACK,
    )));

    let plane = ViewPlane::new(128, 128, 1.0);
    let w = World::new(s, plane, RegularSampler::new(plane));

    let flag = std::env::args().nth(1).unwrap_or("--display".to_string());

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
                std::io::stdout().flush().unwrap();
                render_count += 1;
                if render_count % 100 == 0 {
                    print!("\n");
                }
                w.render_to(&mut CanvasRender::new(image));
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
