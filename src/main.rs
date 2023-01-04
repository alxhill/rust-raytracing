#![allow(dead_code)]

mod types;

use image::ImageBuffer;
use types::Point2D;


fn main() {
    println!("Starting execution.");
    let img = ImageBuffer::from_fn(128, 128, |x, y| {
        image::Rgb([x as u8, y as u8, ((x+y)/2) as u8])
    });

    let v: Point2D = Point2D::new(0.0, 0.0);

    img.save("test.png").unwrap();
}
