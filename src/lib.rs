use wasm_bindgen::{prelude::*, JsCast};
use num_complex::Complex;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};


// This is the Rust function that generates the mandelbrot set. It is
// marked with the #[wasm_bindgen] attribute to indicate that it should
// be exposed to JavaScript.
// #[wasm_bindgen]
pub fn mandelbrot(c: Complex<f64>) -> u8 {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..100 {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return i;
        }
    }

    100
}

// This is a JavaScript function that calls the mandelbrot function and
// uses the `canvas` API to render the resulting set on a `canvas`
// element.
#[wasm_bindgen]
pub fn draw_mandelbrot_set() -> Result<(), JsValue> {
    // Get the `canvas` element and its rendering context.
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("mandelbrot-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Iterate over each point in the complex plane and draw a pixel on
    // the canvas with a color based on the number of iterations it
    // took to determine that the point does not belong to the set.
    for i in -50..50 {
        for j in -50..50 {
            let c = Complex::new(i as f64 / 50.0, j as f64 / 50.0);
            let i = mandelbrot(c);
            if i < 100 {
                let color = format!("rgb({}, 0, {})", 255 - i * 2, i * 2);
                context.set_fill_style(&JsValue::from_str(&color));
                context.fill_rect(i as f64 + 50.0, j as f64 + 50.0, 1.0, 1.0);
            }
        }
    }

    Ok(())
}
