# MandelRust (Mandelbrot Set With Rust)

<!-- Import the mandelbrot.png from assets/mandelbrot.png -->
![Mandelbrot Set](assets/mandelbrot.png)

This project is a Rust implementation of the Mandelbrot set that can be used in a web browser using the WebAssembly technology. The set is generated using the mandelbrot function and rendered on an HTML canvas element using the `draw_mandelbrot_set` function.

Usage
To use this project, you will need a web browser that supports WebAssembly and the canvas API. You can then include the mandelbrot.wasm file in your HTML page and call the `draw_mandelbrot_set` function to render the Mandelbrot set on a canvas element.

Here is an example of how to use the `draw_mandelbrot_set` function in JavaScript:

```js
// Import the draw_mandelbrot_set function from the mandelbrot module.
import { draw_mandelbrot_set } from "./mandelbrot.wasm";

// Get a reference to the canvas element.
const canvas = document.getElementById("mandelbrot-canvas");

// Draw the Mandelbrot set on the canvas with a zoom level of 1.
draw_mandelbrot_set(1.0).then(() => {
    // The set has been successfully drawn on the canvas.
});
```

## Configuration:
---
The `draw_mandelbrot_set` function accepts a zoom parameter that can be used to zoom in or out of the set. A larger zoom value will make the set appear smaller and a smaller zoom value will make the set appear larger.

The function also uses some constant values to calculate the real and imaginary parts of the complex `c` value used in the mandelbrot function. These constants are:

- `WIDTH`: The width of the canvas in pixels.
- `HEIGHT`: The height of the canvas in pixels.
- `RE_START`: The start value of the real part of the `c` value.
- `RE_END`: The end value of the real part of the `c` value.
- `IM_START`: The start value of the imaginary part of the `c` value.
- `IM_END`: The end value of the imaginary part of the `c` value.

You can adjust these constants to change the size and the range of the complex plane that is used to generate the set.

## Development
---
To build this project, you will need the Rust programming language and the `wasm-pack` tool installed on your computer. 

You can then use the following commands to build the project:

```
# Install the required dependencies.
cargo install wasm-pack

# Build the project and generate the mandelbrot.wasm file.
wasm-pack build
```
You can then use the generated mandelbrot.wasm file in your web project to render the Mandelbrot set on a canvas element.

## License
---
This project is licensed under the MIT License. See the LICENSE file for details.