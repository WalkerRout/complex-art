use image::Rgb;
use nalgebra::{Complex, Normed};

use crate::Renderable;

pub struct Julia;

impl Julia {
  const ITERATIONS: usize = 900;

  #[inline(always)]
  fn colour(i: usize) -> Rgb<u8> {
    let i: u8 = i.try_into().unwrap_or(u8::MAX);
    let min = (i < 20) as u8;
    let max = (i > 150) as u8;
    Rgb([
      ((-(i as f64/9.0 - 255.0/18.0).powf(2.0) + 165.0) as u8)
        .saturating_sub(max*20)
        .saturating_add(min*i.saturating_pow(2)),
      (i.saturating_div(4) * i.ilog10() as u8)
        .saturating_add(min*3*i)
        .saturating_sub(max*100),
      (i.saturating_pow(3).ilog10() as u8),
    ])
  }

  #[inline(always)]
  fn default_colour() -> Rgb<u8> {
    Rgb([60, 10, 10])
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
    // -0.802+0.156i
    // -0.4+0.60i
    // -0.17+0.657i
    // -1.38+0.0i
    let c = Complex::new(-0.05, 0.68);
    for i in 0..Self::ITERATIONS {
      if z.norm() > 2.0 {
        return Self::colour(i);
      }
      z = z * z + c;
    }
    Self::default_colour()
  }
}
