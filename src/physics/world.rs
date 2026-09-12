use crate::{
    color::Color,
    computing::computing::Computing,
    math::{matrix::Matrix, point::Point},
    physics::{
        intersect::Intersection, material::*, object::Object, ray::Ray,
        shape_collection::sphere::Sphere,
    },
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
    fn shade_hit(&self, comps: Computing, remaining: u32) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);

        let surface = comps.object.material().lighting(
            &self.light,
            comps.point,
            comps.eyev,
            comps.normalv,
            shadowed,
            Some(&comps.object),
        );

        let reflected = self.reflected_color(comps.clone(), remaining);
        let refracted = self.refracted_color(comps.clone(), remaining);

        let material = comps.object.material();

        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();

            surface
                + reflected * reflectance
                + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }
    pub fn color_at(&self, ray: Ray, remaining: u32) -> Color {
        let xs = self.intersect_world(ray);
        let hit = Intersection::hit(&xs);

        if hit.is_none() {
            return Color::new(0.0, 0.0, 0.0);
        }

        let comps = Computing::prepare_computations(&hit.unwrap(), ray, &xs);

        self.shade_hit(comps, remaining)
    }
    pub fn reflected_color(&self, comps: Computing, remaining: u32) -> Color {
        if remaining == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if comps.object.material().reflective == 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(comps.over_point, comps.relectv);

        let color = self.color_at(reflect_ray, remaining - 1);

        color * comps.object.material().reflective
    }
    pub fn refracted_color(&self, comps: Computing, remaining: u32) -> Color {
        if remaining == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if comps.object.material().transparency == 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot_product(comps.normalv);

        let sin2_t = n_ratio.powf(2.0) * (1.0 - cos_i.powf(2.0));

        if sin2_t > 1.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let cos_t = (1.0 - sin2_t).sqrt();

        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

        let refracted_ray = Ray::new(comps.under_point, direction);

        let c = self.color_at(refracted_ray, remaining - 1) * comps.object.material().transparency;

        c
    }
}

#[cfg(test)]
mod tests {
    use std::{num::IntErrorKind::PosOverflow, ops::Not};

    use crate::{
        computing::computing::Computing,
        math::{matrix::Matrix, vector::Vector},
        physics::{
            material::Material,
            ray::Ray,
            shape_collection::{plane::Plane, sphere::Sphere},
            type_pattern::TypePattern,
        },
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

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);
        let c = w.shade_hit(comps, 5);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c)
    }
    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = Point_Light::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let shape = w.objects[1].clone(); //the second object in w

        let i = Intersection::new(0.5, &shape);

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);
        let c = w.shade_hit(comps, 5);

        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c)
    }
    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        w.objects[0].material_mut().ambient(1.0); // outer
        w.objects[1].material_mut().ambient(1.0); // inner

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let c = w.color_at(r, 5);

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

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);

        let c = w.shade_hit(comps, 5);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn the_reflect_color_for_a_nonreflect_material() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0));
        let i = Intersection::new(w.objects[0].material().ambient, &w.objects[0]);

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);
        let color = w.refracted_color(comps, 5);

        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = shape.transform.translate(0.0, -1.0, 0.0);
        let obj = Object::Plane(shape);
        w.add_object(obj.clone());
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = &Intersection::new(2.0_f64.sqrt(), &obj);

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);

        let color = w.reflected_color(comps, 5);
        assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274))
    }
    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = shape.transform.translate(0.0, -1.0, 0.0);
        let obj = Object::Plane(shape);
        w.add_object(obj.clone());
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = &Intersection::new(2.0_f64.sqrt(), &obj);

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);

        let color = w.shade_hit(comps, 5);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918))
    }
    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::default();
        w.light = Point_Light::new(Point::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
        let mut lower = Plane::new();
        lower.material.reflective = 1.0;
        lower.transform = lower.transform.translate(0.0, -1.0, 0.0);

        let object_lower = Object::Plane(lower);
        w.add_object(object_lower);

        let mut upper = Plane::new();

        upper.material.reflective = 1.0;
        upper.transform = upper.transform.translate(0.0, 1.0, 0.0);

        let object_upper = Object::Plane(upper);

        w.add_object(object_upper);

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));

        let color = w.color_at(ray, 5);
        assert!(true)
    }
    #[test]
    fn the_reflected_color_at_maximum_recursive_depth() {
        let w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = shape.transform.translate(0.0, -1.0, 0.0);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let object_shape = Object::Plane(shape);
        let i = Intersection::new(2.0_f64.sqrt(), &object_shape);

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);

        let color = w.refracted_color(comps, 0);
    }
    #[test]
    fn the_refracted_color_at_maximum_recursive_depth() {
        let mut shape = Sphere::new();

        shape.material.transparency = 0.0;
        shape.material.refractive_index = 1.5;

        let object = Object::Sphere(shape);

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let xs = vec![
            Intersection::new(4.0, &object),
            Intersection::new(6.0, &object),
        ];

        let comps = Computing::prepare_computations(&xs[0], r, &xs);

        let w = World::default();

        let c = w.refracted_color(comps, 0);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let w = World::default();

        let mut shape = w.objects[0].clone();

        shape.material_mut().transparency = 1.0;
        shape.material_mut().refractive_index = 1.5;

        let r = Ray::new(
            Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        let xs = vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, &shape),
            Intersection::new(2.0_f64.sqrt() / 2.0, &shape),
        ];

        let comps = Computing::prepare_computations(&xs[1], r, &xs);

        let c = w.refracted_color(comps, 5);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_with_a_refrected_ray() {
        let mut w = World::default();

        //The first object in world
        w.objects[0].material_mut().ambient = 1.0;
        w.objects[0].material_mut().pattern = Some(TypePattern::test_patter());

        //The second object in world
        w.objects[1].material_mut().transparency = 1.0;
        w.objects[1].material_mut().refractive_index = 1.5;

        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));

        let xs = vec![
            Intersection::new(-0.9899, &w.objects[0]),
            Intersection::new(-0.4899, &w.objects[1]),
            Intersection::new(0.4899, &w.objects[1]),
            Intersection::new(0.9899, &w.objects[0]),
        ];
        let comps = Computing::prepare_computations(&xs[2], r, &xs);

        let c = w.refracted_color(comps, 5);

        assert_eq!(c, Color::new(0.0, 0.99888, 0.04725))
    }
    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = World::default();

        let mut floor = Plane::new();
        floor.transform = floor.transform.translate(0.0, -1.0, 0.0);
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = ball.transform.translate(0.0, -3.5, -0.5);

        let object_floor = Object::Plane(floor);
        let object_ball = Object::Sphere(ball);

        w.add_object(object_ball);
        w.add_object(object_floor.clone());

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0), 
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );

        let xs = vec![
            Intersection::new(2.0_f64.sqrt(), &object_floor)
        ];

        let comps = Computing::prepare_computations(&xs[0], r, &xs);

        let c = w.shade_hit(comps, 5);

        assert_eq!(
            c, 
            Color::new(0.93642, 0.68642, 0.68642)
        )

    }
    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = World::default();

        let mut floor = Plane::new();
        floor.transform = floor.transform.translate(0.0, -1.0, 0.0);
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = ball.transform.translate(0.0, -3.5, -0.5);

        let object_floor = Object::Plane(floor);
        let object_ball = Object::Sphere(ball);

        w.add_object(object_ball);
        w.add_object(object_floor.clone());

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0), 
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );

        let xs = vec![
            Intersection::new(2.0_f64.sqrt(), &object_floor)
        ];

        let comps = Computing::prepare_computations(&xs[0], r, &xs);

        let c = w.shade_hit(comps, 5);

        assert_eq!(
            c, 
            Color::new(0.93391, 0.69643, 0.69243)
        )
    }
}
