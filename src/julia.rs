use image::Rgb;
use nalgebra::{Complex, Normed};

use crate::Renderable;

pub struct Julia;

impl Julia {
  const ITERATIONS: usize = 255;

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

impl Renderable for Julia {
  #[inline(always)]
  fn pixel_to_complex(width: u32, height: u32, x: u32, y: u32) -> Complex<f64> {
    let scale_x = 3.0 / width as f64;
    let scale_y = 2.5 / height as f64;
    let cr = x as f64 * scale_x - (3.0 / 2.0);
    let ci = y as f64 * scale_y - (2.5 / 2.0);
    Complex::new(cr, ci)
  }

  #[inline]
  fn complex_to_colour(mut z: Complex<f64>) -> Rgb<u8> {
    let c = Complex::new(-0.8, 0.156);
    for i in 0..Self::ITERATIONS {
      if z.norm() > 2.0 {
        return Self::colour(i);
      }
      z = z * z + c;
    }
    Self::default_colour()
  }
}
