mod camera;
mod canvas;
mod color;
mod computing;
mod math;
mod physics;
mod utils;

use std::f64::consts::PI;

use crate::color::Color;
use crate::math::matrix::Matrix;
use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::physics::material::Point_Light;
use crate::physics::object::Object;
use crate::physics::patterns_collection::gradient_pattern::Gradient_Pattern;
use crate::physics::patterns_collection::ring_pattern::Ring_Pattern;
use crate::physics::patterns_collection::stripe_patttern::Stripe_Pattern;
use crate::physics::shape_collection::sphere::Sphere;
use crate::physics::type_pattern::TypePattern;
use crate::physics::world::World;
use crate::physics::*;
use crate::utils::view_transformation;
use crate::{camera::camera::Camera, physics::material::Material};

fn main() {
    let mut floor = shape_collection::plane::Plane::new();
    floor.material.color(Color::new(1.0, 0.9, 0.9));
    floor.material.specular(0.0);

    let ring = Ring_Pattern::new();

    floor.material.pattern = Some(TypePattern::Ring_Pattern(ring));

    let mut middle = Sphere::new();
    middle.transform = Matrix::traslation(-0.5, 1.0, 0.5);
    middle.material = Material::default();
    middle.material.color(Color::new(0.1, 1.0, 0.5));
    middle.material.diffuse(0.7);
    middle.material.specular(0.3);
    //middle.material.pattern = Some(TypePattern::Ring_Pattern(ring));

    let mut right = Sphere::new();
    right.material = Material::default();
    right.material.color(Color::new(0.5, 1.0, 0.1));
    right.material.diffuse(0.7);
    right.material.specular(0.3);

    let mut left = Sphere::new();
    left.material = Material::default();
    left.material.color(Color::new(1.0, 0.8, 0.1));
    left.material.diffuse(0.7);
    left.material.specular(0.3);

    right.transform =
        (Matrix::traslation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5)).unwrap();
    left.transform =
        (Matrix::traslation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33)).unwrap();

    let mut w = World::default();
    w.light = Point_Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    w.objects.remove(0);
    w.objects.remove(0);
    w.add_object(Object::Plane(floor));
    // w.add_object(Object::Sphere(left_wall));
    // w.add_object(Object::Sphere(right_wall));
    w.add_object(Object::Sphere(middle));
    w.add_object(Object::Sphere(right));
    w.add_object(Object::Sphere(left));

    let mut c = Camera::new(1000, 500, PI / 3.0);
    c.transform = view_transformation(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canv = c.render_screen(w);
    canv.canvas_to_ppm().unwrap();
}
