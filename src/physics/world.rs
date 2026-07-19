use crate::{
    color::Color,
    math::{matrix::Matrix, point::Point},
    physics::{material::*, sphere::Sphere},
};

pub struct World {
    light: Point_Light,
    objects: Vec<Sphere>,
}

impl World {
    pub fn default() -> Self {
        let light = Point_Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut m1 = Material::default();
        m1.color(Color::new(0.8, 1.0, 0.6));
        m1.diffuse(0.7);
        m1.specular(0.2);

        let mut s1 = Sphere::new();
        s1.set_material(m1);

        let mut s2 = Sphere::new();
        s2.set_transform(Matrix::identity(4).scale(0.5, 0.5, 0.5));

        World {
            light,
            objects: vec![s1, s2],
        }
    }
    pub fn add_object(&mut self, object: Sphere) {
        self.objects.push(object);
    }
}

#[cfg(test)]

mod tests {
    use crate::{
        math::matrix::Matrix,
        physics::{material::Material, sphere},
    };

    use super::*;

    #[test]
    fn the_default_world() {
        let w = World::default();
        let light = Point_Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut m1 = Material::default();
        m1.color(Color::new(0.8, 1.0, 0.6));
        m1.diffuse(0.7);
        m1.specular(0.2);

        let mut s1 = Sphere::new();
        s1.set_material(m1);

        let mut s2 = Sphere::new();
        s2.set_transform(Matrix::identity(4).scale(0.5, 0.5, 0.5));

        assert_eq!(w.light, light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }
}
