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
use std::sync::Arc;

fn main() {
    println!("Starting execution.");
    let mut scene = Scene::new();

    let red_mat = Arc::new(Phong::reflective(0.25, 0.65, 0.6, 20.0, RGBColor::RED, 0.5));
    let yellow_mat = Arc::new(Phong::new(0.2, 0.8, 0.0, 1.0, RGBColor::YELLOW));
    let grey_mat = Arc::new(Phong::new(0.5, 0.5, 0.0, 1.0, RGBColor::GREY));
    let green_mat = Arc::new(Phong::reflective(0.2, 0.4, 0.0, 1.0, RGBColor::GREEN, 0.8));

    scene.add_object(Object::sphere(Sphere::new(Point3D::zero(), 30.0), red_mat));
    scene.add_object(Object::sphere(
        Sphere::new(Point3D::new(0.0, 60.0, -1.0), 20.0),
        yellow_mat,
    ));
    scene.add_object(Object::sphere(
        Sphere::new(Point3D::new(-40.0, 25.0, -2.0), 15.0),
        green_mat.clone(),
    ));
    scene.add_object(Object::sphere(
        Sphere::new(Point3D::new(40.0, 15.0, -2.0), 15.0),
        green_mat,
    ));
    scene.add_object(Object::plane(
        Plane::new(Point3D::new(0.0, -50.0, 0.0), Vector3D::new(0.0, 1.0, 0.0)),
        grey_mat.clone(),
    ));
    scene.add_object(Object::plane(
        Plane::new(Point3D::new(0.0, 0.0, 150.0), Vector3D::new(0.0, 0.0, -1.0)),
        grey_mat,
    ));

    scene.add_light(Light::point_light(Point3D::new(0.0, 100.0, 0.0), 2.0));
    scene.add_light(Light::point_light(Point3D::new(0.0, 0.0, -50.0), 2.0));

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

            println!("Starting display.");
            canvas.render(move |_, image| {
                // copy_to(&render.buffer, &mut CanvasTarget::new(image));
                camera.position().move_by(&Vector3D::new(0.0, 0.0, 0.5));
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