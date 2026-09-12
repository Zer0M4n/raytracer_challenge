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
use crate::physics::patterns_collection::checker3d_pattern::Checker3DPattern;
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

    // ============================================================
    // FLOOR
    // ============================================================

    let mut floor = shape_collection::plane::Plane::new();

    floor.material.color(
        Color::new(0.08, 0.08, 0.10)
    );

    floor.material.ambient(0.20);
    floor.material.diffuse(0.70);
    floor.material.specular(0.20);


    // ============================================================
    // BACK WALL
    // ============================================================

    let mut back_wall = shape_collection::plane::Plane::new();

    back_wall.material.color(
        Color::new(0.05, 0.12, 0.30)
    );

    back_wall.material.ambient(0.30);
    back_wall.material.diffuse(0.60);
    back_wall.material.specular(0.10);

    back_wall.transform =
        (Matrix::traslation(0.0, 0.0, 5.0)
        * Matrix::rotation_x(PI / 2.0)).unwrap();


    // ============================================================
    // LEFT WALL
    // ============================================================

    let mut left_wall = shape_collection::plane::Plane::new();

    left_wall.material.color(
        Color::new(0.25, 0.04, 0.12)
    );

    left_wall.material.ambient(0.25);
    left_wall.material.diffuse(0.65);
    left_wall.material.specular(0.15);

    left_wall.transform =
        (Matrix::traslation(-6.0, 0.0, 0.0)
        * Matrix::rotation_z(PI / 2.0)).unwrap();


    // ============================================================
    // RIGHT WALL
    // ============================================================

    let mut right_wall = shape_collection::plane::Plane::new();

    right_wall.material.color(
        Color::new(0.05, 0.25, 0.12)
    );

    right_wall.material.ambient(0.25);
    right_wall.material.diffuse(0.65);
    right_wall.material.specular(0.15);

    right_wall.transform =
        (Matrix::traslation(6.0, 0.0, 0.0)
        * Matrix::rotation_z(-PI / 2.0)).unwrap();


    // ============================================================
    // 1. CENTER - PERFECT MIRROR
    // ============================================================
    // The main object of the scene.
    // Maximum reflection.

    let mut center = Sphere::new();

    center.transform =
        (Matrix::traslation(0.0, 1.8, 0.5)
        * Matrix::scaling(1.8, 1.8, 1.8)).unwrap();

    center.material.color(
        Color::new(0.015, 0.018, 0.025)
    );

    center.material.ambient(0.0);
    center.material.diffuse(0.0);
    center.material.specular(1.0);
    center.material.shininess(600.0);

    center.material.reflective = 1.0;


    // ============================================================
    // 2. RED GLASS - LEFT FRONT
    // ============================================================

    let mut red_glass = Sphere::glass_sphere();

    red_glass.transform =
        (Matrix::traslation(-3.5, 0.85, -0.8)
        * Matrix::scaling(0.85, 0.85, 0.85)).unwrap();

    red_glass.material.color(
        Color::new(0.85, 0.01, 0.015)
    );

    red_glass.material.ambient(0.05);
    red_glass.material.diffuse(0.10);
    red_glass.material.specular(1.0);
    red_glass.material.shininess(450.0);

    red_glass.material.reflective = 0.75;
    red_glass.material.transparency = 0.90;
    red_glass.material.refractive_index = 1.50;


    // ============================================================
    // 3. GOLD SPHERE - RIGHT FRONT
    // ============================================================

    let mut gold = Sphere::new();

    gold.transform =
        (Matrix::traslation(3.5, 0.60, -0.5)
        * Matrix::scaling(0.60, 0.60, 0.60)).unwrap();

    gold.material.color(
        Color::new(1.0, 0.55, 0.02)
    );

    gold.material.ambient(0.10);
    gold.material.diffuse(0.75);
    gold.material.specular(0.90);
    gold.material.shininess(220.0);


    // ============================================================
    // 4. WATER - FAR LEFT
    // ============================================================

    let mut water = Sphere::water_sphere();

    water.transform =
        (Matrix::traslation(-3.6, 1.45, 2.4)
        * Matrix::scaling(1.45, 1.45, 1.45)).unwrap();

    water.material.color(
        Color::new(0.02, 0.25, 1.0)
    );

    water.material.ambient(0.03);
    water.material.diffuse(0.08);
    water.material.specular(1.0);
    water.material.shininess(400.0);

    water.material.reflective = 0.20;
    water.material.transparency = 0.95;


    // ============================================================
    // 5. PURPLE - FAR RIGHT
    // ============================================================

    let mut purple = Sphere::new();

    purple.transform =
        (Matrix::traslation(3.6, 1.15, 2.5)
        * Matrix::scaling(1.15, 1.15, 1.15)).unwrap();

    purple.material.color(
        Color::new(0.60, 0.03, 0.95)
    );

    purple.material.ambient(0.12);
    purple.material.diffuse(0.75);
    purple.material.specular(0.80);
    purple.material.shininess(180.0);


    // ============================================================
    // 6. SAPPHIRE - BACK LEFT
    // ============================================================

    let mut sapphire = Sphere::sapphire_sphere();

    sapphire.transform =
        (Matrix::traslation(-2.3, 0.55, 4.0)
        * Matrix::scaling(0.55, 0.55, 0.55)).unwrap();

    sapphire.material.color(
        Color::new(0.02, 0.12, 1.0)
    );

    sapphire.material.ambient(0.03);
    sapphire.material.diffuse(0.08);
    sapphire.material.specular(1.0);
    sapphire.material.shininess(500.0);

    sapphire.material.reflective = 0.40;
    sapphire.material.transparency = 0.95;


    // ============================================================
    // 7. DIAMOND - BACK RIGHT
    // ============================================================

    let mut diamond = Sphere::diamond_sphere();

    diamond.transform =
        (Matrix::traslation(2.4, 0.40, 4.2)
        * Matrix::scaling(0.40, 0.40, 0.40)).unwrap();

    diamond.material.color(
        Color::new(0.70, 0.90, 1.0)
    );

    diamond.material.ambient(0.02);
    diamond.material.diffuse(0.04);
    diamond.material.specular(1.0);
    diamond.material.shininess(700.0);

    diamond.material.reflective = 0.70;
    diamond.material.transparency = 1.0;


    // ============================================================
    // WORLD
    // ============================================================

    let mut w = World::default();

    w.objects.clear();


    // ============================================================
    // LIGHT
    // ============================================================

    w.light = Point_Light::new(
        Point::new(-4.0, 8.0, -6.0),
        Color::new(1.4, 1.4, 1.4),
    );


    // ============================================================
    // SCENE OBJECTS
    // ============================================================

    w.add_object(Object::Plane(floor));

    w.add_object(Object::Plane(back_wall));
    w.add_object(Object::Plane(left_wall));
    w.add_object(Object::Plane(right_wall));

    w.add_object(Object::Sphere(center));

    w.add_object(Object::Sphere(red_glass));
    w.add_object(Object::Sphere(gold));
    w.add_object(Object::Sphere(water));
    w.add_object(Object::Sphere(purple));
    w.add_object(Object::Sphere(sapphire));
    w.add_object(Object::Sphere(diamond));


    // ============================================================
    // CAMERA
    // ============================================================

    let mut camera = Camera::new(
        1000,
        600,
        PI / 3.0,
    );

    camera.transform = view_transformation(
        Point::new(0.0, 5.2, -10.0),
        Point::new(0.0, 1.5, 1.5),
        Vector::new(0.0, 1.0, 0.0),
    );


    // ============================================================
    // RENDER
    // ============================================================

    let canvas = camera.render_screen(w);

    canvas.canvas_to_ppm().unwrap();
}