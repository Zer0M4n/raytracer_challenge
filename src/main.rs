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
          let v1 = vec![1.0 , 2.0 , 3.0 , 4.0 ,
                                2.0 , 4.0 , 4.0 , 2.0 ,
                                8.0 , 6.0 , 4.0 , 1.0 ,
                                0.0 , 0.0 , 0.0 , 1.0 ];
        let tuple = vec![1.0,2.0,3.0,1.0];
        let r = vec![18.0,24.0,33.0,1.0];
        let re = Matrix::from_vec(1, 4, r).unwrap();
        let m = Matrix::from_vec(4, 4, v1).unwrap();
        let matrix_tuple = Matrix::from_vec(4, 1, tuple).unwrap();
        let boss =(m * matrix_tuple).unwrap();

    println!("{:?}", boss);
}
