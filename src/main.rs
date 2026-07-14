mod canvas;
mod color;
mod math;
mod physics;
mod utils;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::physics::ray::{self, Ray};
use crate::physics::sphere::Sphere;

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10;
    let wall_size = 7.0;

    let canvas_pixel = 100;

    let pixel_size = wall_size / canvas_pixel as f64;
    let half = wall_size / 2.0;

    let mut c = Canvas::new(canvas_pixel, canvas_pixel);
    let col = Color::new(1.0, 0.0, 0.0);

    let shape = Sphere::new();

    for y in 0..canvas_pixel - 1 {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixel - 1 {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z as f64);
            let v = position - ray_origin;
            let r = Ray::new(ray_origin, v.normalization());
            let xs = shape.intersect(r);
            //let  = ;
            if xs.len() == 2 {
                c.write_pixel(x, y, col);
            }
        }
    }

    c.canvas_to_ppm();
}
