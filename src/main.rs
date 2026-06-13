mod canvas;
mod color;
mod math;
mod utils;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::math::matrix;
use crate::math::point::Point;
use crate::math::vector::Vector;
use core::f64;
use std::convert;
use std::fs::File;
use std::io::{BufWriter, Write};
fn main() -> std::io::Result<()> {
    //println!("Hello, world!");
    let file = File::create("image_example.ppm")?;
    let mut w = BufWriter::new(file);

    let image_width: i64 = 256;
    let image_height: i64 = 256;

    writeln!(w, "P3\n {image_width} {image_height} \n255\n")?;

    let mut j = 0;

    let mut ir = 0;
    let mut ig = 0;
    let mut ib = 0;

    while j < image_height {
        let mut i = 0;

        while i < image_width {
            let r = (i as f64) / (image_width - 1) as f64;
            let g = (j as f64) / (image_width - 1) as f64;
            let b = 0.0;

            ir = (255.999 * r) as i64;
            ig = (255.999 * g) as i64;
            ib = (255.999 * b) as i64;
            i += 1;

            writeln!(w, "{ir} {ig} {ib}")?;
        }
        j += 1;
    }

    Ok(())
}
