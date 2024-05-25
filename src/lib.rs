use image::{ImageBuffer, Rgb};
use nalgebra::Complex;

pub mod julia;
pub mod mandelbrot;

pub trait Renderable {
  fn pixel_to_complex(width: u32, height: u32, x: u32, y: u32) -> Complex<f64>;
  fn complex_to_colour(c: Complex<f64>) -> Rgb<u8>;
}

pub fn generate_image<R>(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>>
where
  R: Renderable,
{
  let mut buffer = ImageBuffer::new(width, height);
  for (x, y, px) in buffer.enumerate_pixels_mut() {
    let complex = R::pixel_to_complex(width, height, x, y);
    *px = R::complex_to_colour(complex);
  }
  buffer
}
