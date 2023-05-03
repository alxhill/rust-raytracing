import * as wasm from "rt-wasm";
import { memory } from "rt-wasm/rt_wasm_bg";

let scene = wasm.JsScene.new();


let pixels_array = new Uint8ClampedArray(memory.buffer, scene.pixels(), scene.width() * scene.height() * 4);
let image_data = new ImageData(pixels_array, scene.width(), scene.height());

let canvas = document.getElementById("raytracing");
canvas.width = scene.width();
canvas.height = scene.height();

let ctx = canvas.getContext("2d");

let render_loop = false;


function toggleRender(e) {
    render_loop = !render_loop;
    if (render_loop) {
        drawRender();
    } else {
        requestAnimationFrame(drawPaused);
    }
}

canvas.addEventListener("click", toggleRender);
canvas.addEventListener("touchstart", toggleRender);

const drawRender = () => {
    const start = performance.now();
    scene.render();
    console.log(`rendering took ${performance.now() - start} ms`);

    ctx.putImageData(image_data, 0, 0);

    scene.move_camera(0, 0, 1.0);

    if (render_loop)
        requestAnimationFrame(drawRender);
}

const drawPaused = () => {
    ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
    ctx.globalCompositeOperation = 'multiply';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.restore();

    ctx.globalCompositeOperation = 'source-over';
    ctx.fillStyle = 'white';
    ctx.font = 'bold 24px Arial';
    ctx.textAlign = 'center';
    ctx.fillText('Click to render', canvas.width / 2, canvas.height / 2);
}

drawRender();
drawPaused();