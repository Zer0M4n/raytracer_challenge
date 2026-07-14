use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::physics::intersect::Intersection;
use crate::physics::ray::Ray;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    radius: f64,
    center: Point,
}

impl Sphere {
    pub fn new(radius: f64) -> Self {
        let center = Point::new(0.0, 0.0, 0.0);
        Sphere { radius, center }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        //the sphere always center
        let sphere_to_ray = ray.origin - self.center;
        let a = Vector::dot_product(&ray.direction, ray.direction);
        let b = 2.0 * Vector::dot_product(&ray.direction, sphere_to_ray);
        let c = Vector::dot_product(&sphere_to_ray, sphere_to_ray) - self.radius * self.radius;

        let discrimant = (b * b) - 4.0 * a * c;

        if discrimant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discrimant.sqrt()) / (2.0 * a);
        let t2 = (-b + discrimant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, self), Intersection::new(t2, self)]
    }
    pub fn transform(&self, m: &Matrix) -> Self {
        
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::null;

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new(1.0);

        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.object, &s));
    }
    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(1.0);

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(std::ptr::eq(xs[0].object, &s));
        assert!(std::ptr::eq(xs[1].object, &s));
    }
    #[test]
    fn intersections_have_positive_t() {
        let s = Sphere::new(1.0);
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        let i = Intersection::hit(&xs); //collection the intersections
        assert_eq!(i.unwrap().t, 1.0);
    }
    #[test]
    fn when_some_intersections_have_negative_t() {
        let s = Sphere::new(1.0);
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2];
        let i = Intersection::hit(&xs); //collection the intersections
        assert_eq!(i.unwrap().t, 1.0);
    }
    #[test]
    fn when_all_intersections_have_negative_t() {
        let s = Sphere::new(1.0);
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i1, i2];
        let i = Intersection::hit(&xs); //collection the intersections
        assert!(i.is_none())
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new(1.0);
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let xs = vec![i1, i2, i3, i4];
        let i = Intersection::hit(&xs); //collection the intersections
        assert_eq!(i.unwrap().t, 2.0);
    }
}
