import * as wasm from "rt-wasm";
import { memory } from "rt-wasm/rt_wasm_bg";

let scene = wasm.JsScene.new();


let pixels_array = new Uint8ClampedArray(memory.buffer, scene.pixels(), scene.width() * scene.height() * 4);
let image_data = new ImageData(pixels_array, scene.width(), scene.height());

let canvas = document.getElementById("raytracing");
canvas.width = scene.width();
canvas.height = scene.height();

let ctx = canvas.getContext("2d");

var render_loop = true;

canvas.addEventListener("click", function (e) {
    render_loop = !render_loop;
    if (render_loop) {
        drawRender();
    }
});

const drawRender = () => {
    const start = performance.now();
    scene.render();
    console.log(`rendering took ${performance.now() - start} ms`);

    ctx.putImageData(image_data, 0, 0);

    scene.move_camera(0, 0, 1.0);

    if (render_loop)
        requestAnimationFrame(drawRender);
}

drawRender();