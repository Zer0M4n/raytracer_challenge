use crate::math::point::Point;
use crate::math::vector::Vector;
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin:Point , direction:Vector) -> Self {
        Ray { origin, direction }
    }
    
}