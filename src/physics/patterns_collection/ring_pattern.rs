use crate::{
    color::Color,
    math::{matrix::Matrix, point::Point},
    physics::object::Object,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Ring_Pattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Ring_Pattern {
    pub fn new() -> Self {
        let a = Color::new(1.0, 0.0, 0.0);
        let b = Color::new(0.0, 0.0, 1.0);
        let transform: Matrix = Matrix::scaling(0.25, 0.25, 0.25);

        Ring_Pattern { a, b, transform }
    }
    pub fn ring_at(&self, point: Point) -> Color {
        let x = (point.x.powf(2.0) + point.z.powf(2.0)).sqrt();
        if x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
    pub fn ring_at_object(&self, object: &Object, world_point: Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * world_point;

        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.ring_at(pattern_point)
    }
}
