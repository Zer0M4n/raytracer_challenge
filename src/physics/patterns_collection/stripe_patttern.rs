use crate::{
    color::Color,
    math::{matrix::Matrix, point::Point},
    physics::object::Object,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Stripe_Pattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Stripe_Pattern {
    pub fn new() -> Self {
        Stripe_Pattern {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.0, 0.0, 0.0),
            transform: Matrix::scaling(0.25, 0.25, 0.25),
        }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
    pub fn stripe_at_object(&self, object: &Object, world_point: Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * world_point;

        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.stripe_at(pattern_point)
    }
}
