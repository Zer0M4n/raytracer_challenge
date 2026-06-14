mod canvas;
mod color;
mod math;
mod utils;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::math::matrix::{self, Matrix4};
use crate::math::point::Point;
use crate::math::vector::Vector;
use std::fmt::Display;

fn main() {
    //println!("Hello, world!");
    let data = [
        [1.0, 2.0, 3.0, 2.0],
        [4.0, 5.0, 6.0, 5.0],
        [7.0, 9.0, 5.0, 7.0],
        [1.0, 2.0, 3.0, 3.0],
    ];

    let matrix = Matrix4::new(data);
}
