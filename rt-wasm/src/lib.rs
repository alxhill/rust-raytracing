use rust_raytracing::prelude::*;
use std::fmt::Display;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Display for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rgba({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

#[wasm_bindgen]
pub fn render() -> JsBuffer {
    console_error_panic_hook::set_once();
    let mut s = Scene::new();
    let red_mat = Arc::new(Phong::new(0.2, 0.3, 1.0, 5.0, RGBColor::RED));
    s.add_object(Object::sphere(Sphere::new(Point3D::zero(), 50.0), red_mat));
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
        let mut buffer = Vec::with_capacity((width * height) as usize);
        buffer.resize((width * height) as usize, Rgba(1, 1, 1, 1));
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
        let rgba: Rgba = color.into();
        log(&format!("{}", rgba));
        self.buffer[(xy.x() + xy.y() * self.w) as usize] = color.into();
    }
}

impl From<&RGBColor> for Rgba {
    fn from(value: &RGBColor) -> Self {
        Rgba(
            (value.r * 255.0) as u8,
            (value.g * 255.0) as u8,
            (value.b * 255.0) as u8,
            0,
        )
    }
}
