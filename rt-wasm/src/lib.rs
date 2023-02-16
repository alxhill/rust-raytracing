use rust_raytracing::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}, you're a dickhead", name));
}
//
// #[wasm_bindgen]
// #[derive(Clone, Copy)]
// pub struct Rgb(u8, u8, u8);
//
// #[wasm_bindgen]
// pub fn pixel(x: u32, y: u32) -> Rgb {
//     let mut s = Scene::new();
//     let red_mat = Arc::new(Phong::reflective(0.2, 0.3, 1.0, 5.0, RGBColor::RED, 1.0));
//     s.add_object(Object::sphere(Sphere::new(Point3D::zero(), 10.0), red_mat));
//     s.add_light(Light::point_light(Point3D::new(0.0, 100.0, 0.0), 2.0));
//
//     let plane = ViewPlane::new(128, 128, 1.0);
//     let camera = PinholeCamera::new(-100.0, 100.0);
//     let sampler = JitteredSampler::new(plane, 1);
//
//     let mut buffer = JsBuffer::new(plane.width as usize, plane.height as usize);
//
//     render_to(&s, &plane, &sampler, &camera, &mut buffer);
//
//     buffer.buffer[x as usize][y as usize]
// }
//
// struct JsBuffer {
//     buffer: Vec<Vec<Rgb>>,
// }
//
// impl JsBuffer {
//     fn new(width: usize, height: usize) -> Self {
//         let mut buffer = Vec::with_capacity(width);
//         for _ in 0..width {
//             let mut row = Vec::with_capacity(height);
//             for _ in 0..height {
//                 row.push(Rgb(0, 0, 0));
//             }
//             buffer.push(row);
//         }
//         Self { buffer }
//     }
// }
//
// impl RenderTarget for JsBuffer {
//     fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
//         self.buffer[xy.x() as usize][xy.y() as usize] = Rgb(
//             (color.r * 255.0) as u8,
//             (color.g * 255.0) as u8,
//             (color.b * 255.0) as u8,
//         );
//     }
// }
