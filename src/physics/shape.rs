use crate::{
    math::{matrix::Matrix, point::Point, vector::Vector},
    physics::{material::Material, ray::Ray, world},
};

#[derive(Debug, Clone)]
pub struct Shape {
    transform: Matrix,
    material: Material,
    pub saved_ray: Option<Ray>, //The value exist or not , to be is to do
}

impl Shape {
    pub fn test_shape() -> Self {
        Shape {
            transform: Matrix::identity(4),
            material: Material::default(),
            saved_ray: None,
        }
    }

    pub fn intersect(&mut self, ray: Ray) {
        let inverse = self.transform.inverse().unwrap();

        let local_ray = ray.transform(&inverse);

        self.saved_ray = Some(local_ray);
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        point - Point::new(0.0, 0.0, 0.0)
    }
    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = self.transform.inverse().unwrap() * point;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * local_normal;
        //world_normal.w = 0.0;

        world_normal.normalization()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::math::{point::Point, vector::Vector};

    use super::*;
    #[test]
    fn the_default_transformation() {
        let s = Shape::test_shape();
        assert_eq!(s.transform, Matrix::identity(4))
    }

    #[test]
    fn the_default_mateerial() {
        let s = Shape::test_shape();
        assert_eq!(s.material, Material::default())
    }
    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::test_shape();
        s.transform = s.transform.scale(2.0, 2.0, 2.0);

        let xs = s.intersect(r);

        assert_eq!(s.saved_ray.unwrap().origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(s.saved_ray.unwrap().direction, Vector::new(0.0, 0.0, 0.5));
    }
    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Shape::test_shape();
        s.transform = s.transform.translate(5.0, 0.0, 0.0);

        let xs = s.intersect(r);

        assert_eq!(s.saved_ray.unwrap().origin, Point::new(-5.0, 0.0, -5.0));
        assert_eq!(s.saved_ray.unwrap().direction, Vector::new(0.0, 0.0, 1.0));
    }
    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = Shape::test_shape();
        s.transform = s.transform.translate(0.0, 1.0, 0.0);

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711_f64, -0.70711_f64))
    }
    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = Shape::test_shape();
        s.transform = s.transform.scale(1.0, 0.5, 1.0).rotate_z(PI / 5.0);

        let n = s.normal_at(Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254))
    }
}
