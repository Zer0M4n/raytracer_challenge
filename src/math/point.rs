use crate::math::vector::Vector;
use crate::utils::comparing_floating_number;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

//The logic point , w is 1.0 for math facilities
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    fn equal_point(&self, v2: &Point) -> bool {
        comparing_floating_number(self.x, v2.x)
            && comparing_floating_number(self.y, v2.y)
            && comparing_floating_number(self.z, v2.z)
    }
    fn add_vector(&self, other: &Vector) -> Self {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    fn subtrac_point(&self, other: &Self) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    fn vector_subtrac_point(&self, other: Vector) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    fn multi_scalar(&self, scalar: f64) -> Self {
        Point::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
    fn div_scalar(&self, scalar: f64) -> Self {
        Point::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

//Arithmetic Operators Overload
impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        self.add_vector(&rhs)
    }
}
impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtrac_point(&rhs)
    }
}
impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        self.vector_subtrac_point(rhs)
    }
}
impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        self.multi_scalar(rhs)
    }
}
impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        self.div_scalar(rhs)
    }
}
impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

//Boolean Operators Overload
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.equal_point(other)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poin_does_stored_values() {
        let point = Point::new(4.3, -4.2, 3.1);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
    }
    #[test]
    fn point_equal_point() {
        let p = Point::new(4.3, -4.2, 3.1);
        let p2 = Point::new(4.3, -4.200001, 3.1);

        assert!(p.equal_point(&p2));
        assert!(p == p2);
    }
    #[test]
    fn point_add_vector() {
        let p = Point::new(1.0, 2.0, 3.0);
        let p2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(
            p + p2,
            Point {
                x: 3.0,
                y: 5.0,
                z: 7.0,
                w: 1.0
            }
        )
    }
    #[test]
    fn point_subtrac_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p - p2, Vector::new(-2.0, -4.0, -6.0))
    }

    #[test]
    fn p_sub_v() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0))
    }
    #[test]
    fn point_negation() {
        let p = Point::new(3.0, 2.0, 1.0);

        assert_eq!(-p, Point::new(-3.0, -2.0, -1.0))
    }
    #[test]
    fn scalar_multi_point() {
        let p = Point::new(1.0, -2.0, 3.0);

        assert_eq!(p * 3.5, Point::new(3.5, -7.0, 10.5));
        assert_eq!(p * 0.5, Point::new(0.5, -1.0, 1.5));
    }
    #[test]
    fn scalar_div_point() {
        let p = Point::new(1.0, -2.0, 3.0);

        assert_eq!(p / 2.0, Point::new(0.5, -1.0, 1.5));
    }
}
