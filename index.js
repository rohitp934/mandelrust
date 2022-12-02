// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
import init, { draw_mandelbrot_set } from "./pkg/mandelrust.js";

async function run() {
  console.log("Loading wasm...");
  await init();
  console.log("Loaded wasm!");

  // Set the background color of the page and the canvas to black.
  document.body.style.background = "black";
  const canvas = document.getElementById("mandelbrot-canvas");
  // @ts-ignore
  const context = canvas.getContext("2d");
  context.fillStyle = "white";

  // Initialize the zoom factor.
  let zoom = 1.0;

  // Define a function that updates the zoom level and redraws the
  // Mandelbrot set at the new zoom level.
  function update() {
    // Increment the zoom level.
    zoom *= 1.1;

    // Draw the Mandelbrot set at the new zoom level.
    draw_mandelbrot_set(zoom);

    // Request the next animation frame.
    window.requestAnimationFrame(update);
  }

  // Start the animation loop by requesting the first animation frame.
  window.requestAnimationFrame(update);
}

run();
