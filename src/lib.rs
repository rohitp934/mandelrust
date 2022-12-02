use num_complex::Complex;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};


const MAX_ITER: u32 = 80;

// This is the Rust function that generates the mandelbrot set. It is
// marked with the #[wasm_bindgen] attribute to indicate that it should
// be exposed to JavaScript.
// #[wasm_bindgen]
pub fn mandelbrot(c: Complex<f64>, zoom: f64) -> u32 {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..MAX_ITER {
        z = z * z + c;
        if z.norm_sqr() > 4.0 / zoom {
            return i;
        }
    }

    MAX_ITER
}

// This is a JavaScript function that calls the mandelbrot function and
// uses the `canvas` API to render the resulting set on a `canvas`
// element.
#[wasm_bindgen]
pub fn draw_mandelbrot_set(zoom: f64) -> Result<(), JsValue> {
    const WIDTH: usize = 600;
    const HEIGHT: usize = 400;
    const RE_START: f64 = -2.0;
    const RE_END: f64 = 1.0;
    const IM_START: f64 = -1.0;
    const IM_END: f64 = 1.0;

    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("mandelbrot-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Set the canvas dimensions to 1280x720.
    canvas.set_width(WIDTH as u32);
    canvas.set_height(HEIGHT as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Clear the canvas.
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    // Iterate over each point in the complex plane and draw a pixel on
    // the canvas with a color based on the number of iterations it
    // took to determine that the point does not belong to the set.
    
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let re = RE_START + i as f64 * (RE_END - RE_START) / WIDTH as f64;
            let im = IM_START + j as f64 * (IM_END - IM_START) / HEIGHT as f64;
            let c = Complex::new(re, im);
            let i = mandelbrot(c, zoom);
            // let h = (i * 255) % MAX_ITER;
            // let s = 255;
            // let v = if i < MAX_ITER { 255 } else { 0 };
            let r = if i < MAX_ITER { 0 } else { i*2 };
            let g = 0;
            let b = if i < MAX_ITER { 0 } else { 255 - i*2 };
            // let color = format!("hsl({}, {}%, {}%)", h, s, v);
            let color = format!("rgb({}, {}, {})", r, g, b);
            context.set_fill_style(&JsValue::from_str(&color));
            context.fill_rect(i as f64, j as f64, 1.0, 1.0);
        }
    }

    Ok(())
}
