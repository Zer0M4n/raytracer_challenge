use std::f64::EPSILON;

use crate::{
    math::{point::Point, vector::Vector},
    physics::{intersect::Intersection, object::Object, ray::Ray, shape_collection::sphere::Sphere},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Computing<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl<'a> Computing<'a> {
    pub fn prepare_computations(intersection: &Intersection<'a>, ray: Ray) -> Self {
        let comp_p = ray.position(intersection.t);

        let mut normalv = intersection.object.normal_at(comp_p);

        let inside;

        if normalv.dot_product(-ray.direction) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        const EPSILON: f64 = 0.0001;

        let over_point = comp_p + normalv * EPSILON;

        Computing {
            t: intersection.t,
            object: intersection.object,
            point: comp_p,
            eyev: -ray.direction,
            normalv,
            inside,
            over_point,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let object = Object::Sphere(shape);

        let i = Intersection::new(4.0, &object);

        let comps = Computing::prepare_computations(&i, r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let object = Object::Sphere(shape);

        let i = Intersection::new(4.0, &object);

        let comps = Computing::prepare_computations(&i, r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let object = Object::Sphere(shape);

        let i = Intersection::new(1.0, &object);

        let comps = Computing::prepare_computations(&i, r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
}
