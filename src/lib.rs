use num_complex::Complex;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// This is the Rust function that generates the mandelbrot set. It is
// marked with the #[wasm_bindgen] attribute to indicate that it should
// be exposed to JavaScript.
// #[wasm_bindgen]
pub fn mandelbrot(c: Complex<f64>, zoom: f64) -> u32 {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..1000 {
        z = z * z + c;
        if z.norm_sqr() > 4.0 / zoom {
            return i;
        }
    }

    100
}

// This is a JavaScript function that calls the mandelbrot function and
// uses the `canvas` API to render the resulting set on a `canvas`
// element.
#[wasm_bindgen]
pub fn draw_mandelbrot_set(zoom: f64) -> Result<(), JsValue> {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("mandelbrot-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Set the canvas dimensions to 1280x720.
    canvas.set_width(1280);
    canvas.set_height(720);

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
    for i in -100..100 {
        for j in -100..100 {
            let c = Complex::new(i as f64 / 50.0, j as f64 / 50.0);
            let i = mandelbrot(c, zoom);
            // if i < 100 {
            //     let color = format!("rgb({}, 0, {})", i * 2, 255 - i * 2);
            //     context.set_fill_style(&JsValue::from_str(&color));
            //     context.fill_rect(i as f64 + 50.0, j as f64 + 50.0, 1.0, 1.0);
            // }
            if i < 100 {
                // Set the color of the points that belong to the set.
                let color = "rgb(0, 0, 0)";
                context.set_fill_style(&JsValue::from_str(&color));
                context.fill_rect(i as f64 + 50.0, j as f64 + 50.0, 1.0, 1.0);
            } else {
                // Set the color of the points that do not belong to the set.
                let color = format!("rgb({}, 0, {})", i * 2, 255 - i * 2);
                context.set_fill_style(&JsValue::from_str(&color));
                context.fill_rect(i as f64 + 50.0, j as f64 + 50.0, 1.0, 1.0);
            }
        }
    }

    Ok(())
}
