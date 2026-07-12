use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::physics::intersect::Intersection;
use crate::physics::ray::Ray;
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
        let c = Vector::dot_product(&sphere_to_ray, sphere_to_ray) - 1.0;

        let discrimant = (b * b) - 4.0 * a * c;

        if discrimant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discrimant.sqrt()) / (2.0 * a);
        let t2 = (-b + discrimant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, self), Intersection::new(t2, self)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new(1.0);

        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.object, &s));
    }
}
