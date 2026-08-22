use crate::{
    color::Color,
    computing::computing::Computing,
    math::{matrix::Matrix, point::Point},
    physics::{intersect::Intersection, material::*, object::Object, ray::Ray, shape_collection::sphere::Sphere},
};

pub struct World {
    pub light: Point_Light,
    pub objects: Vec<Object>,
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
            objects: vec![Object::Sphere(s1), Object::Sphere(s2)],
        }
    }
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
    pub fn intersect_world(&self, r: Ray) -> Vec<Intersection<'_>> {
        let mut xs = Vec::new();

        for object in &self.objects {
            xs.extend(object.intersect(r));
        }

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        xs
    }
    fn is_shadowed(&self, p: Point) -> bool {
        let v = self.light.point - p;
        let distance = v.lenght();
        let direction = v.normalization();

        let r = Ray::new(p, direction);

        let intersections = self.intersect_world(r);

        if let Some(hit) = Intersection::hit(&intersections) {
            hit.t < distance
        } else {
            false
        }
    }
    fn shade_hit(&self, comps: Computing) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);

        comps.object.material().lighting(
            &self.light,
            comps.point,
            comps.eyev,
            comps.normalv,
            shadowed,
            Some(&comps.object),
        )
    }
    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect_world(ray);
        let hit = Intersection::hit(&xs);

        if hit.is_none() {
            return Color::new(0.0, 0.0, 0.0);
        }

        let comps = Computing::prepare_computations(&hit.unwrap(), ray);

        self.shade_hit(comps)
    }
}

#[cfg(test)]
mod tests {
    use std::{num::IntErrorKind::PosOverflow, ops::Not};

    use crate::{
        computing::computing::Computing,
        math::{matrix::Matrix, vector::Vector},
        physics::{material::Material, ray::Ray, shape_collection::sphere::Sphere},
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
        assert!(w.objects.contains(&Object::Sphere(s1)));
        assert!(w.objects.contains(&Object::Sphere(s2)));
    }
    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let xs = w.intersect_world(r);

        assert_eq!(xs.len(), 4);

        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[0].clone(); //First object in w
        let i = Intersection::new(4.0, &shape);
        let comps = Computing::prepare_computations(&i, r);
        let c = w.shade_hit(comps);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c)
    }
    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = Point_Light::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let shape = w.objects[1].clone(); //the second object in w

        let i = Intersection::new(0.5, &shape);

        let comps = Computing::prepare_computations(&i, r);
        let c = w.shade_hit(comps);

        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c)
    }
    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        w.objects[0].material_mut().ambient(1.0); // outer
        w.objects[1].material_mut().ambient(1.0); // inner

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let c = w.color_at(r);

        assert_eq!(c, w.objects[1].material().color);
    }
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p))
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);

        assert!(w.is_shadowed(p).not())
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);

        assert!(w.is_shadowed(p).not())
    }
    #[test]
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::default();

        w.light = Point_Light::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        // Eliminar las dos esferas del mundo por defecto
        w.objects.clear();

        // Primera esfera
        let s1 = Sphere::new();
        w.add_object(Object::Sphere(s1));

        // Segunda esfera
        let mut s2 = Sphere::new();
        s2.transform = s2.transform.translate(0.0, 0.0, 10.0);

        w.add_object(Object::Sphere(s2));

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));

        let i = Intersection::new(4.0, &w.objects[1]);

        let comps = Computing::prepare_computations(&i, r);

        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
