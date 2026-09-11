use crate::{
    color::Color,
    math::{matrix::Matrix, point::Point},
    physics::object::Object,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TestPattern {
    pub transform: Matrix,
}

impl TestPattern {
    pub fn new() -> Self {
        Self {
            transform: Matrix::identity(4),
        }
    }

    pub fn test_at(&self, point: Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }

    pub fn test_at_object(&self, object: &Object, world_point: Point) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.test_at(pattern_point)
    }
}
