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
            let maybe_hit = obj.hit(&Ray::new(Point3D::zero(), Vector3D::zero()), &mut tmin);
            match maybe_hit {
                Some(hit) => {
                    return image::Rgb(hit.color.to_u8())
                },
                None => {}
            }
        }
        image::Rgb(RGBColor::BLACK.to_u8())
    });

    img.save("test.png").unwrap();
}
