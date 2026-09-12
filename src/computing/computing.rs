use crate::{
    math::{point::Point, vector::Vector},
    physics::{
        intersect::Intersection, object::Object, ray::Ray, shape_collection::sphere::Sphere,
    },
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
    pub under_point: Point,
    pub relectv: Vector,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Computing<'a> {
    pub fn prepare_computations(
        intersection: &Intersection<'a>,
        ray: Ray,
        xs: &[Intersection<'a>],
    ) -> Self {
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
        let under_point = comp_p - normalv * EPSILON;
        // Índices de refracción
        let mut n1 = 1.0;
        let mut n2 = 1.0;

        // Objetos que actualmente contienen al rayo
        let mut containers: Vec<&Object> = Vec::new();

        for i in xs {
            // ¿Es esta la intersección que estamos preparando?
            if std::ptr::eq(i, intersection) {
                if let Some(object) = containers.last() {
                    n1 = object.material().refractive_index;
                }
            }

            // Si el objeto ya está dentro de containers,
            // significa que estamos saliendo de él.
            if let Some(index) = containers
                .iter()
                .position(|object| std::ptr::eq(*object, i.object))
            {
                containers.remove(index);
            } else {
                // Si no está, estamos entrando.
                containers.push(i.object);
            }

            // Después de actualizar containers obtenemos n2
            if std::ptr::eq(i, intersection) {
                if let Some(object) = containers.last() {
                    n2 = object.material().refractive_index;
                }

                break;
            }
        }

        Computing {
            t: intersection.t,
            object: intersection.object,
            point: comp_p,
            eyev: -ray.direction,
            normalv,
            inside,
            over_point,
            under_point,
            relectv: ray.direction.reflect(normalv),
            n1,
            n2,
        }
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot_product(self.normalv);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powf(2.0) * (1.0 - cos.powf(2.0));
            
            if sin2_t > 1.0 {
                return 1.0;
            }
            let cos_t = (1.0 - sin2_t).sqrt();

            cos = cos_t;
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powf(2.0);

        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::EPSILON;

    use crate::physics::shape_collection::plane::Plane;

    use super::*;

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let object = Object::Sphere(shape);

        let i = Intersection::new(4.0, &object);

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);

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

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let object = Object::Sphere(shape);

        let i = Intersection::new(1.0, &object);

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
    #[test]
    fn precomputing_the_reflecting_vector() {
        let shape = Plane::new();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let object = Object::Plane(shape);

        let i = Intersection::new(2.0_f64.sqrt(), &object);

        let xs = vec![i.clone()];

        let comps = Computing::prepare_computations(&i, r, &xs);

        assert_eq!(
            comps.relectv,
            Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        )
    }
    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::glass_sphere();
        shape.transform = shape.transform.translate(0.0, 0.0, 1.0);
        let object_shape = Object::Sphere(shape);
        let i = Intersection::new(5.0, &object_shape);
        let xs = vec![i.clone()];
        let comps = Computing::prepare_computations(&i, r, &xs);

        assert!(comps.under_point.z > 0.0001 / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(
            Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0), 
            Vector::new(0.0, 1.0, 0.0)
        );
        let object_shape = Object::Sphere(shape);
        let xs = vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, &object_shape),
            Intersection::new(2.0_f64.sqrt() / 2.0, &object_shape),
        ];

        let comps = Computing::prepare_computations(&xs[1], r, &xs);

        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0)
    }
    #[test]
    fn the_schilick_aproximation_with_small_angle_an_n2_mayor_n1() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(
            Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0), 
            Vector::new(0.0, 1.0, 0.0)
        );
        let object_shape = Object::Sphere(shape);
        let xs = vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, &object_shape),
            Intersection::new(2.0_f64.sqrt() / 2.0, &object_shape),
        ];

        let comps = Computing::prepare_computations(&xs[1], r, &xs);

        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0)
    }

}
