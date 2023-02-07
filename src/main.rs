#![allow(dead_code)]
extern crate image;
extern crate pixel_canvas;

mod render;
mod types;
mod world;

use std::sync::Arc;
use crate::render::{render_to, CanvasTarget, ImageTarget};
use crate::types::*;
use crate::world::*;
use pixel_canvas::input::MouseState;
use pixel_canvas::Canvas;

fn main() {
    println!("Starting execution.");
    let mut scene = Scene::new();

    let red_mat = Arc::new(Phong::new(0.25, 0.65, 0.6, 20.0, RGBColor::RED));
    let yellow_mat = Arc::new(Phong::new(0.2, 0.8, 0.0, 1.0, RGBColor::YELLOW));
    let grey_mat = Arc::new(Phong::new(0.5, 0.5, 0.0, 1.0, RGBColor::GREY));
    let green_mat = Arc::new(Phong::new(0.2, 0.7, 1.0, 100.0, RGBColor::GREEN));

    let red_sphere = Object::sphere(Sphere::new(Point3D::zero(), 30.0), red_mat);
    let yellow_sphere = Object::sphere(Sphere::new(Point3D::new(0.0, 60.0, -1.0), 20.0), yellow_mat);

    let green_sphere_1 = Object::sphere(Sphere::new(Point3D::new(-40.0, 25.0, -2.0), 15.0), green_mat.clone());
    let green_sphere_2 = Object::sphere(Sphere::new(Point3D::new(40.0, 15.0, -2.0), 15.0), green_mat);

    let plane = Object::plane(Plane::new(Point3D::new(0.0, -50.0, 0.0), Vector3D::new(0.0, 1.0, 0.0)), grey_mat.clone());
    let back_plane = Object::plane(Plane::new(Point3D::new(0.0, 0.0, 150.0), Vector3D::new(0.0, 0.0, -1.0)), grey_mat);

    let light1 = Light::point_light(Point3D::new(0.0, 100.0, 0.0), 2.0);
    let light2 = Light::point_light(Point3D::new(0.0, 0.0, -50.0), 2.0);

    scene.add_object(red_sphere);
    scene.add_object(yellow_sphere);
    scene.add_object(plane);
    scene.add_object(back_plane);
    scene.add_object(green_sphere_1);
    scene.add_object(green_sphere_2);
    scene.add_light(light1);
    scene.add_light(light2);

    let plane = ViewPlane::new(512, 512, 0.5);
    let mut camera = PinholeCamera::new(-100.0, 100.0);
    let sampler = JitteredSampler::new(plane, 8);

    let flag = std::env::args().nth(1).unwrap_or("--display".to_string());

    match flag.as_str() {
        "--display" => {
            let canvas = Canvas::new(plane.width as usize, plane.height as usize)
                .title("Ray Tracer")
                .show_ms(true)
                .state(MouseState::new())
                .input(MouseState::handle_input);

            // let mut render_count: u32 = 0;
            println!("Starting display.");
            canvas.render(move |_, image| {
                // copy_to(&render.buffer, &mut CanvasTarget::new(image));
                // print!(".");
                // std::io::stdout().flush().unwrap();
                // render_count += 1;
                // if render_count % 100 == 0 {
                //     println!();
                // }
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
