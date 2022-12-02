use num_complex::Complex;

pub fn mandelbrot(c: Complex<f64>) -> u8 {
  let mut z = Complex::new(0.0, 0.0);
  // let mut i = 0;

  for i in 0..100 {
      z = z * z + c;
      if z.norm_sqr() > 4.0 {
          return i;
      }
  }

  100
}

fn main() {
  let c = Complex::new(-50 as f64 / 50.0, -16 as f64 / 50.0);
  let i = mandelbrot(c);
  println!("i: {}", i);
}