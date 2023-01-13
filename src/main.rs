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
            let mut tmin: Double = f64::MAX;
            let maybeHit = obj.hit(&Ray::new(Point3D::zero(), Vector3D::zero()), &mut tmin);
            match maybeHit {
                Some(hit) => {
                    return image::Rgb([hit.color.r as u8, hit.color.g as u8, hit.color.b as u8])
                },
                None => {}
            }
        }
        image::Rgb([x as u8, y as u8, ((x+y)/2) as u8])
    });

    img.save("test.png").unwrap();
}
