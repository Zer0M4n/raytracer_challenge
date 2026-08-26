use crate::{
    color::Color,
    math::{matrix::Matrix, point::Point},
    physics::object::Object,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Checker3DPattern {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Checker3DPattern {
    pub fn new() -> Self {
        let a = Color::new(0.0, 0.0, 1.0);
        let b = Color::new(1.0, 1.0, 1.0);
let transform = Matrix::scaling(0.25, 0.25, 0.25);
        Checker3DPattern { a, b, transform  }
    }
pub fn checker_at(&self, point: Point) -> Color {
    let sum =
        point.x.floor() as i64 +
        point.y.floor() as i64 +
        point.z.floor() as i64;

    if sum.rem_euclid(2) == 0 {
        self.a
    } else {
        self.b
    }
}
    pub fn checker_at_object(&self, object: &Object, world_point: Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * world_point;

        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.checker_at(pattern_point)
    }
}
