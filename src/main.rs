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
use std::io::SeekFrom::Start;
use std::io::{BufWriter, Write};
#[derive(Debug, Clone, Copy)]
pub struct projectile {
    position: Point,
    velocity: Vector,
}
#[derive(Debug, Clone, Copy)]
struct enviroment {
    gravity: Vector,
    wind: Vector
}

fn tick(env: enviroment, proj: projectile) -> projectile {

    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.wind + env.gravity;

    projectile { position, velocity }
    
}
fn main() {
    //println!("Hello, world!");
    let start = Point::new(0.0,1.0,0.0);
    let velocity_vector = Vector::new(1.0,1.8,0.0);
    let velocity = velocity_vector.normalization() * 11.25 ;

    let red = Color::new(1.0, 0.0, 0.0);
    
    let gravity = Vector::new(0.0,-1.0,0.0) ;
    let wind = Vector::new(-0.01, 0.0, 0.0) ;

    let e = enviroment{gravity,wind} ;
    let mut position = start ;
    let mut p = projectile{position , velocity};
    let mut c = Canvas::new(900, 550) ;

    while p.position.y >= 0.0 {
        let x = p.position.x as u64;
        let y = 549 - p.position.y as u64;

        if x < 900 && y < 550 {
            c.write_pixel(x, y, red);
        }

        p = tick(e, p);
    }

    c.canvas_to_ppm();

}
