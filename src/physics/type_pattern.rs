use crate::{color::Color, math::point::Point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypePattern {
    Stripe_Pattern(Stripe_Pattern),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stripe_Pattern {
    a: Color,
    b: Color,
}

impl Stripe_Pattern {
    pub fn new() -> Self {
        Stripe_Pattern {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

impl TypePattern {
    pub fn at(&self, point: Point) -> Color {
        match self {
            TypePattern::Stripe_Pattern(stripe) => stripe.stripe_at(point),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let stripe = Stripe_Pattern::new();

        assert_eq!(TypePattern::Stripe_Pattern(stripe).at(Point::new(0.0, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(TypePattern::Stripe_Pattern(stripe).at(Point::new(0.9, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(TypePattern::Stripe_Pattern(stripe).at(Point::new(1.0, 0.0, 0.0)), Color::new(0.0, 0.0, 0.0));
    }
}
