#![allow(dead_code)]

mod types;
mod world;

use image::ImageBuffer;
use crate::types::Point3D;
use crate::types::Vector3D;
use crate::types::Ray;
use crate::world::World;
use crate::world::Sphere;

fn main() {
    println!("Starting execution.");

    let mut w = World::new();
    w.add(Box::new(Sphere::new(Point3D::zero(), 1.0)));

    let img = ImageBuffer::from_fn(128, 128, |x, y| {
        for obj in w.objects() {
            let maybeHit = obj.hit(Ray::new(Point3D::zero(), Vector3D::zero()), 0.0);
        }
        image::Rgb([x as u8, y as u8, ((x+y)/2) as u8])
    });

    img.save("test.png").unwrap();
}
