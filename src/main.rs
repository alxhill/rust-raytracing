#![allow(dead_code)]
extern crate image;
extern crate pixel_canvas;

mod render;
mod types;
mod world;

use crate::render::{render_to, CanvasRender, ImageRender};
use crate::types::*;
use crate::world::*;
use pixel_canvas::input::MouseState;
use pixel_canvas::Canvas;
use std::io::Write;

fn main() {
    println!("Starting execution.");

    let mut scene = Scene::new();

    scene.add(Box::new(Sphere::new(Point3D::zero(), 40.0, RGBColor::RED)));
    scene.add(Box::new(Sphere::new(
        Point3D::new(0.0, 20.0, -1.0),
        30.0,
        RGBColor::YELLOW,
    )));
    scene.add(Box::new(Plane::new(
        Point3D::new(0.0, -50.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        RGBColor::GREY,
    )));

    let plane = ViewPlane::new(256, 256, 0.5);
    let camera = PerspectiveCamera::new(-100.0, 100.0);
    let sampler = JitteredSampler::new(plane, 4);

    let flag = std::env::args().nth(1).unwrap_or("--display".to_string());

    match flag.as_str() {
        "--display" => {
            let canvas = Canvas::new(plane.width as usize, plane.height as usize)
                .title("Tile")
                .show_ms(true)
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
                render_to(
                    &scene,
                    &plane,
                    &sampler,
                    &camera,
                    &mut CanvasRender::new(image),
                );
            });
        }
        "--output" => {
            let mut render = ImageRender::new(plane.width, plane.height);
            render_to(&scene, &plane, &sampler, &camera, &mut render);
            render.save_image("output.png".to_string());
        }
        _ => println!("Invalid flag: {}", flag),
    }
}
