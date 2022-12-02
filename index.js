// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
import init, { draw_mandelbrot_set } from "./pkg/mandelrust.js";

async function run() {
  console.log("Loading wasm...");
  await init();
  console.log("Loaded wasm!");

  draw_mandelbrot_set();
}

run();
