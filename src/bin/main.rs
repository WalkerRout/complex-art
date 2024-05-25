use complex_art::generate_image;
#[allow(unused_imports)]
use complex_art::julia::Julia;
#[allow(unused_imports)]
use complex_art::mandelbrot::Mandelbrot;

fn main() {
  let (width, height) = (10000, 10000);
  let image_buffer = generate_image::<Julia>(width, height);
  image_buffer
    .save("fractal.png")
    .expect("unable to save as png");
}
