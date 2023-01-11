#![allow(dead_code)]

mod types;
mod world;

use image::ImageBuffer;
use crate::types::Point3D;
use crate::world::World;
use crate::world::Sphere;

fn main() {
    println!("Starting execution.");
    let img = ImageBuffer::from_fn(128, 128, |x, y| {
        image::Rgb([x as u8, y as u8, ((x+y)/2) as u8])
    });

    let mut w = World::new();
    w.add(Box::new(Sphere::new(Point3D::zero(), 1.0)));

    img.save("test.png").unwrap();
}
