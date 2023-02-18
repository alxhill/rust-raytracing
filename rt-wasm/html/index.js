import * as wasm from "rt-wasm";
import { memory } from "rt-wasm/rt_wasm_bg";

const start = performance.now();
let output = wasm.render();
console.log(`rendering took ${performance.now() - start} ms`);

let pixel_start_ptr = output.pixels();
let pixels_array = new Uint8ClampedArray(memory.buffer, pixel_start_ptr, output.width() * output.height() * 4);

let canvas = document.getElementById("raytracing");
canvas.width = output.width();
canvas.height = output.height();

let ctx = canvas.getContext("2d");

const drawRender = () => {
    console.log("drawing to screen");
    ctx.putImageData(new ImageData(pixels_array, output.width(), output.height()), 0, 0);
}

drawRender();