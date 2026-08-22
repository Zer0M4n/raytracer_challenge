use crate::{
    color::Color, math::{matrix::Matrix, point::Point}, physics::{object::Object, patterns_collection::{self, stripe_patttern::Stripe_Pattern}, world::World},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypePattern {
    Stripe_Pattern(Stripe_Pattern),
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
