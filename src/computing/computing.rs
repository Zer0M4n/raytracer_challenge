use crate::{
    math::{point::Point, vector::Vector},
    physics::{intersect::Intersection, ray::Ray, sphere::Sphere},
};

#[derive(Debug, Clone, PartialEq, Copy)]
struct Computing<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
}

impl<'a> Computing<'a> {
    pub fn prepare_computations(intersection: &Intersection<'a>, ray: Ray) -> Self {
        let comp_p = ray.position(intersection.t);
        Computing {
            t: intersection.t,
            object: intersection.object,
            point: comp_p,
            eyev: -ray.direction,
            normalv: intersection.object.normal_at(comp_p),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Precomputing_the_state_of_an_intersection() {
        let r = Ray::new(
            Point::new(0.0, 0.0, -5.0), 
            Vector::new(0.0, 0.0, 1.0)
        );
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        let comps = Computing::prepare_computations(&i, r);

        assert_eq!(comps.t , i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
}