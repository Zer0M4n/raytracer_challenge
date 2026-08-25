use crate::{
    color::Color,
    math::{matrix::Matrix, point::Point},
    physics::{
        object::Object,
        patterns_collection::{
            self, gradient_pattern::Gradient_Pattern, ring_pattern::Ring_Pattern,
            stripe_patttern::Stripe_Pattern,
        },
        world::World,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypePattern {
    Stripe_Pattern(Stripe_Pattern),
    Gradient_Pattern(Gradient_Pattern),
    Ring_Pattern(Ring_Pattern),
}

impl TypePattern {
    pub fn test_patter() -> TypePattern {
        let p = Stripe_Pattern::new();

        TypePattern::Stripe_Pattern(p)
    }
    pub fn at(&self, object: &Object, world_point: Point) -> Color {
        match self {
            TypePattern::Stripe_Pattern(stripe) => stripe.stripe_at_object(object, world_point),
            TypePattern::Gradient_Pattern(gradient) => {
                gradient.gradient_at_object(object, world_point)
            }
            TypePattern::Ring_Pattern(ring) => ring.ring_at_object(object, world_point),
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
