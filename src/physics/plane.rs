use std::f64::EPSILON;

use crate::{
    math::{matrix::Matrix, point::Point, vector::Vector},
    physics::{material::Material, ray::Ray},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    pub material: Material,
    pub transform: Matrix,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            material: Material::default(),
            transform: Matrix::identity(4),
        }
    }
    pub fn local_normal_at(&self, p: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
    pub fn local_intersect(&self, ray: Ray) -> Vec<f64> {
        if ray.direction.y.abs() < EPSILON {
            return Vec::new();
        }

        let t = -ray.origin.y / ray.direction.y;

        vec![t]
    }
}

#[cfg(test)]
mod tests {

    use crate::physics::ray::Ray;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();

        let n1 = p.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }
    #[test]
    fn intersect_with_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let xs = p.local_intersect(r);

        assert!(xs[0].is_finite())
    }
    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let xs = p.local_intersect(r);

        assert!(xs[0].is_finite())
    }
}
