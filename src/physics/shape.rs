use crate::{math::matrix::Matrix, physics::{material::Material, ray::Ray}};

#[derive(Debug)]
pub struct Shape {
    transform: Matrix,
    material: Material,
}
#[derive(Debug)]
struct local_intersect<'a> {
    shape: &'a Shape,
    local_ray: Ray,
}

impl Shape {
    pub fn test_shape() -> Self{
        let transform = Matrix::identity(4);
        let material = Material::default();
        Shape { transform, material }
    }
    pub fn intersect(&self, ray: Ray) -> local_intersect<'_> {
        let shape = self;
        let local_ray = ray.transform(&self.transform.inverse().unwrap());
        local_intersect { shape, local_ray }
    }
}

#[cfg(test)]
mod tests {
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
}