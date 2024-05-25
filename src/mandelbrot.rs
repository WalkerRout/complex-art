use image::Rgb;
use nalgebra::{Complex, Normed};

use crate::Renderable;

pub struct Mandelbrot;

impl Mandelbrot {
  const ITERATIONS: usize = 900;

  #[inline(always)]
  fn colour(i: usize) -> Rgb<u8> {
    Rgb([
      (i / 200) as u8 * 255,
      (i / 30) as u8 * 10,
      (i / 10) as u8 * 3,
    ])
  }

  #[inline(always)]
  fn default_colour() -> Rgb<u8> {
    Rgb([10; 3])
  }
}

impl Renderable for Mandelbrot {
  #[inline(always)]
  fn pixel_to_complex(width: u32, _: u32, x: u32, y: u32) -> Complex<f64> {
    let w = width as f64;
    let cr = (x as f64 - (0.75 * w)) / (w / 2.0);
    let ci = (y as f64 - (w / 2.0)) / (w / 2.0);
    Complex::new(cr, ci)
  }

  #[inline]
  fn complex_to_colour(c: Complex<f64>) -> Rgb<u8> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..Self::ITERATIONS {
      if z.norm() > 2.0 {
        return Self::colour(i);
      }
      z = z * z + c;
    }
    Self::default_colour()
  }
}
