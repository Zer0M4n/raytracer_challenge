use crate::physics::{object::Object, shape_collection::sphere::Sphere};

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Self {
        Self { t, object }
    }

    pub fn hit(xs: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
        xs.iter()
            .filter(|i| i.t >= 0.0)
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
