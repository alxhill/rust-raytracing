#![allow(dead_code)]

mod types;
mod world;

use crate::types::*;
use crate::world::*;
use image::ImageBuffer;

fn main() {
    println!("Starting execution.");

    let mut w = World::new();
    w.add(Box::new(Sphere::new(
        Point3D::new(0.0, 0.0, 1.0),
        1.0,
        RGBColor::RED,
    )));

    let img = w.render_to_image();

    img.save("test.png").unwrap();
}
