use std::fmt::Alignment::Center;

use crate::math::point::Point;
use crate::math::vector::{self, Vector};
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }
    pub fn position(&self, time: f64) -> Point {
        self.origin + self.direction * time
    }
}

struct Sphere {
    radius: f64,
    center: Point,
}

impl Sphere {
    fn new(radius: f64) -> Self {
        let center = Point::new(0.0, 0.0, 0.0);
        Sphere { radius, center }
    }

    fn intersect(&self, ray: Ray) -> Vec<f64> {
        //the sphere always center
        let sphere_to_ray = ray.origin - self.center;
        let a = Vector::dot_product(&ray.direction, ray.direction);
        let b = 2.0 * Vector::dot_product(&ray.direction, sphere_to_ray);
        let c = Vector::dot_product(&sphere_to_ray, sphere_to_ray) - 1.0;

        let discrimant = (b * b) - 4.0 * a * c;

        if discrimant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discrimant.sqrt()) / (2.0 * a);
        let t2 = (-b + discrimant.sqrt()) / (2.0 * a);
        vec![t1, t2]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }
    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(1.0);

        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }
    #[test]
    fn ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(1.0);

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }
    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(1.0);

        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(1.0);

        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }
    #[test]
    fn sphere_is_behind_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(1.0);

        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
