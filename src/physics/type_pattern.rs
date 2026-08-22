use crate::{
    color::Color,
    math::{matrix::Matrix, point::Point},
    physics::{object::Object, world::World},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypePattern {
    Stripe_Pattern(Stripe_Pattern),
}

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
            transform: Matrix::identity(4),
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

impl TypePattern {
    pub fn at(&self, object: &Object, world_point: Point) -> Color {
        match self {
            TypePattern::Stripe_Pattern(stripe) => stripe.stripe_at_object(object, world_point),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn a_stripe_pattern_alternates_in_x() {
        let stripe = Stripe_Pattern::new();

        assert_eq!(TypePattern::Stripe_Pattern(stripe.clone())(Point::new(0.0, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(TypePattern::Stripe_Pattern(stripe.clone()).at(Point::new(0.9, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(TypePattern::Stripe_Pattern(stripe.clone()).at(Point::new(1.0, 0.0, 0.0)), Color::new(0.0, 0.0, 0.0));
    }*/
}
