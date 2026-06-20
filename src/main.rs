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
    let v1 = vec![
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    ];
    let mut boss = Matrix::from_vec(4, 4, v1).unwrap();

    println!("{:?}", boss);
    boss.traspose();
    println!("{:?}", boss);
}
