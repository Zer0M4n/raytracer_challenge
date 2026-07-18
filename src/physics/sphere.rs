use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::physics::intersect::Intersection;
use crate::physics::material::Material;
use crate::physics::ray::Ray;
#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: Matrix::identity(4),
            material: Material::default(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection<'_>> {
        //the sphere always center
        let inv = self.transform.inverse().unwrap();
        let ray = ray.transform(&inv);
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
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
    pub fn set_transform(&mut self, m: Matrix) {
        self.transform = m;
    }
    pub fn normal_at(&self, p: Point) -> Vector {
        let mut inverse = self.transform.inverse().unwrap();

        let object_point = inverse.clone() * p;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);

        let world_normal = inverse.transpose() * object_normal;

        world_normal.normalization()
    }
    pub fn set_material(&mut self, m: Material) {
        self.material = m;
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();

        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.object, &s));
    }
    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(std::ptr::eq(xs[0].object, &s));
        assert!(std::ptr::eq(xs[1].object, &s));
    }
    #[test]
    fn intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        let i = Intersection::hit(&xs); //collection the intersections
        assert_eq!(i.unwrap().t, 1.0);
    }
    #[test]
    fn when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2];
        let i = Intersection::hit(&xs); //collection the intersections
        assert_eq!(i.unwrap().t, 1.0);
    }
    #[test]
    fn when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i1, i2];
        let i = Intersection::hit(&xs); //collection the intersections
        assert!(i.is_none())
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let xs = vec![i1, i2, i3, i4];
        let i = Intersection::hit(&xs); //collection the intersections
        assert_eq!(i.unwrap().t, 2.0);
    }
    #[test]
    fn a_sphere_default_transformation() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::identity(4));

        assert_eq!(s.transform, Matrix::identity(4));
    }
    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }
    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transform(Matrix::traslation(5.0, 0.0, 0.0));

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0)
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = Sphere::normal_at(&s, Point::new(1.0, 0.0, 0.0));

        assert_eq!(n, Vector::new(1.0, 0.0, 0.0))
    }
    #[test]
    fn Computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::identity(4).translate(0.0, 1.0, 0.0));

        let n = unsafe { s.normal_at(Point::new(0.0, 1.70711, -0.70711)) };

        unsafe { assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711)) }
    }
    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = (Matrix::identity(4).scale(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0)).unwrap();
        s.set_transform(m);

        let n = s.normal_at(Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254))
    }
}
