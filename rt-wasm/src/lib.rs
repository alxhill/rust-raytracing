use rust_raytracing::prelude::*;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

#[wasm_bindgen]
pub fn render() -> JsBuffer {
    let mut s = Scene::new();
    let red_mat = Arc::new(Phong::reflective(0.2, 0.3, 1.0, 5.0, RGBColor::RED, 1.0));
    s.add_object(Object::sphere(Sphere::new(Point3D::zero(), 10.0), red_mat));
    s.add_light(Light::point_light(Point3D::new(0.0, 100.0, 0.0), 2.0));

    let plane = ViewPlane::new(128, 128, 1.0);
    let camera = PinholeCamera::new(-100.0, 100.0);
    let sampler = JitteredSampler::new(plane, 1);

    let mut buffer = JsBuffer::new(plane.width, plane.height);

    render_to(&s, &plane, &sampler, &camera, &mut buffer);

    buffer
}

#[wasm_bindgen]
pub struct JsBuffer {
    w: u32,
    h: u32,
    buffer: Vec<Rgba>,
}

#[wasm_bindgen]
impl JsBuffer {
    fn new(width: u32, height: u32) -> Self {
        let buffer = Vec::with_capacity((width * height) as usize);
        Self {
            w: width,
            h: height,
            buffer,
        }
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn pixels(&self) -> *const Rgba {
        self.buffer.as_ptr()
    }
}

impl RenderTarget for JsBuffer {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
        self.buffer[(xy.x() + xy.y() * self.w) as usize] = Rgba(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            0 // used by js canvas
        );
    }
}
