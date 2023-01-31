#![allow(dead_code)]
extern crate image;
extern crate pixel_canvas;

mod render;
mod types;
mod world;

use crate::render::{render_to, CanvasTarget, ImageTarget};
use crate::types::*;
use crate::world::*;
use pixel_canvas::input::MouseState;
use pixel_canvas::Canvas;
use std::io::Write;
use std::sync::Arc;

fn main() {
    println!("Starting execution.");

    let mut scene = Scene::new();

    let red_mat = Arc::new(Matte::new(0.25, 0.65, RGBColor::RED));
    let yellow_mat = Arc::new(Matte::new(0.2, 0.8, RGBColor::YELLOW));
    let grey_mat = Arc::new(Matte::new(0.5, 0.5, RGBColor::GREY));
    scene.add_object(Object::new(
        Sphere::new(Point3D::zero(), 40.0),
        red_mat,
    ));
    scene.add_object(Object::new(
        Sphere::new(Point3D::new(0.0, 20.0, -1.0), 30.0),
        yellow_mat
    ));
    scene.add_object(Object::new(
        Plane::new(Point3D::new(0.0, -50.0, 0.0), Vector3D::new(0.0, 1.0, 0.0)),
        grey_mat,
    ));
    scene.add_light(Light::point_light(Point3D::new(50.0, 50.0, 0.0), 3.0));
    scene.add_light(Light::point_light(Point3D::new(-60.0, -100.0, 0.0), 2.0));

    let plane = ViewPlane::new(256, 256, 0.5);
    let mut camera = PinholeCamera::new(-100.0, 100.0);
    let sampler = JitteredSampler::new(plane, 16);

    let flag = std::env::args().nth(1).unwrap_or("--display".to_string());

    match flag.as_str() {
        "--display" => {
            let canvas = Canvas::new(plane.width as usize, plane.height as usize)
                .title("Ray Tracer")
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
                    println!();
                }
                camera.position().move_by(&Vector3D::new(0.0, 0.0, -0.5));
                render_to(
                    &scene,
                    &plane,
                    &sampler,
                    &camera,
                    &mut CanvasTarget::new(image),
                );
            });
        }
        "--output" => {
            let mut render = ImageTarget::new(plane.width, plane.height);
            render_to(&scene, &plane, &sampler, &camera, &mut render);
            render.save_image("output.png".to_string());
        }
        _ => println!("Invalid flag: {flag}"),
    }
}
