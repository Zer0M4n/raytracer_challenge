mod canvas;
mod color;
mod computing;
mod math;
mod physics;
mod utils;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::computing::*;
use crate::math::point::Point;
use crate::physics::intersect::Intersection;
use crate::physics::material::Point_Light;
use crate::physics::ray::Ray;
use crate::physics::sphere::Sphere;

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 1000;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    // Sphere
    let mut shape = Sphere::new();
    shape.material.color(Color::new(1.0, 0.2, 1.0));

    // Light
    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = Point_Light::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let direction = (position - ray_origin).normalization();
            let ray = Ray::new(ray_origin, direction);

            let xs = shape.intersect(ray);

            if let Some(hit) = Intersection::hit(&xs) {
                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;

                let color = hit.object.material.lighting(&light, eye, normal);

                canvas.write_pixel(x, y, color);
            }
        }
    }

    canvas.canvas_to_ppm();
}
