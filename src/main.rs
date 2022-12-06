#![allow(dead_code)]

use image::{ImageBuffer};

type Float = f32;
type Double = f64;

fn main() {
    println!("Starting execution.");
    let img= ImageBuffer::from_fn(128, 128, |x, y| {
        image::Rgb([x as u8, y as u8, ((x+y)/2) as u8])
    });

    img.save("test.png").unwrap();
}
