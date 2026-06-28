mod canvas;
mod color;
mod math;
mod utils;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::vector::Vector;
use std::fmt::Display;

fn main() {
    let transform = Matrix::traslation(5.0, -3.0, 2.0);
    let p = Point::new(-3.0, 4.0, 5.0);
    let presult = transform * p;
    println!("helomd{}", presult.x);
}
