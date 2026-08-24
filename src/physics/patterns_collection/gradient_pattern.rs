use crate::{
    color::Color,
    math::{matrix::Matrix, point::Point},
    physics::object::Object,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Gradient_Pattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Gradient_Pattern {
    pub fn new() -> Self {
        let a = Color::new(1.0, 0.0, 0.0);
        let b = Color::new(0.0, 0.0, 1.0);
        let transform = Matrix::identity(4);

        Gradient_Pattern { a, b, transform }
    }
    pub fn gradient_at(&self, point: Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
    }
    pub fn gradient_at_object(&self, object: &Object, world_point: Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * world_point;

        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.gradient_at(pattern_point)
    }
}
