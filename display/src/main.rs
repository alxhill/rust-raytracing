#![allow(dead_code)]
extern crate image;
extern crate pixel_canvas;

mod render;

use crate::render::canvas::CanvasTarget;
use crate::render::image::ImageTarget;
use pixel_canvas::input::MouseState;
use pixel_canvas::Canvas;
use rust_raytracing::prelude::*;
use rust_raytracing::render::render_to;

fn main() {
    println!("Starting execution.");
    let scene = Scene::default_scene_one();

    let plane = ViewPlane::new(512, 512, 0.5);
    let mut camera = PinholeCamera::new(-100.0, 100.0);
    let mut sampler = MultiJittered::new(9);

    let flag = std::env::args().nth(1).unwrap_or("--display".to_string());

    match flag.as_str() {
        "--display" => {
            let canvas = Canvas::new(plane.width as usize, plane.height as usize)
                .title("Ray Tracer")
                .show_ms(true)
                .state(MouseState::new())
                .input(MouseState::handle_input);

            println!("Starting display.");
            canvas.render(move |_, image| {
                camera.position().move_by(&Vector3D::new(0.0, 0.0, 0.5));
                render_to(
                    &scene,
                    &plane,
                    &mut sampler,
                    &camera,
                    &mut CanvasTarget::new(image),
                );
            });
        }
        "--output" => {
            let mut render = ImageTarget::new(plane.width, plane.height);
            render_to(&scene, &plane, &mut sampler, &camera, &mut render);
            render.save_image("output.png".to_string());
        }
        _ => println!("Invalid flag: {flag}"),
    }
}
