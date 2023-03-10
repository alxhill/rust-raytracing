use rust_raytracing::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

#[wasm_bindgen]
pub struct JsScene {
    scene: Scene,
    plane: ViewPlane,
    sampler: JitteredSampler,
    camera: PinholeCamera,
    target: JsRenderTarget,
}

#[wasm_bindgen]
impl JsScene {
    pub fn new() -> JsScene {
        console_error_panic_hook::set_once();

        let scene = Scene::default_scene_one();

        let plane = ViewPlane::new(512, 512, 0.5);
        let camera = PinholeCamera::new(-100.0, 100.0);
        let sampler = JitteredSampler::new(8);

        let target = JsRenderTarget::new(plane.width, plane.height);

        JsScene {
            scene,
            plane,
            sampler,
            camera,
            target,
        }
    }

    pub fn render(&mut self) {
        render_to(
            &self.scene,
            &self.plane,
            &mut self.sampler,
            &self.camera,
            &mut self.target,
        );
    }

    pub fn move_camera(&mut self, x: Double, y: Double, z: Double) {
        self.camera.position().move_by(&Vector3D::new(x, y, z));
    }

    pub fn pixels(&self) -> *const Rgba {
        self.target.buffer.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.target.width
    }

    pub fn height(&self) -> u32 {
        self.target.height
    }
}

pub struct JsRenderTarget {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<Rgba>,
}

impl JsRenderTarget {
    fn new(width: u32, height: u32) -> JsRenderTarget {
        let mut buffer = Vec::with_capacity((width * height) as usize);
        buffer.resize((width * height) as usize, Rgba(1, 1, 1, 1));
        JsRenderTarget {
            width,
            height,
            buffer,
        }
    }
}

impl RenderTarget for JsRenderTarget {
    fn set_pixel(&mut self, xy: &ViewXY, color: &RGBColor) {
        self.buffer[(xy.x() + (self.height - xy.y() - 1) * self.height) as usize] = color.into();
    }
}

impl From<&RGBColor> for Rgba {
    fn from(value: &RGBColor) -> Self {
        Rgba(
            (value.r * 255.0) as u8,
            (value.g * 255.0) as u8,
            (value.b * 255.0) as u8,
            255,
        )
    }
}
