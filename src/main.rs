#![allow(dead_code)]

mod types;
mod world;

use image::ImageBuffer;
use crate::types::*;
use crate::world::*;

fn main() {
    println!("Starting execution.");

    let mut w = World::new();
    w.add(Box::new(Sphere::new(Point3D::new(0.0, 0.0, 1.0), 1.0, RGBColor::RED)));
    w.add(Box::new(Plane::new(Point3D::zero(), Vector3D::new(0.0, 0.0, 1.0), RGBColor::WHITE)));

    let img = ImageBuffer::from_fn(128, 128, |x, y| {
        for obj in w.objects() {
            let maybeHit = obj.hit(Ray::new(Point3D::zero(), Vector3D::zero()), 0.0);
        }
        image::Rgb([x as u8, y as u8, ((x+y)/2) as u8])
    });

    img.save("test.png").unwrap();
}
