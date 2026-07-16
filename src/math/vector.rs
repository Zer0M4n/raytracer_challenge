use crate::math::point::Point;
use crate::utils::comparing_floating_number;
use std::cmp::PartialEq;
use std::num::FpCategory::Normal;
use std::ops::{Add, Div, Mul, Neg, Sub};
//The logic vector , w is 0.0 for math facilities
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    fn equal_vector(&self, v2: &Vector) -> bool {
        comparing_floating_number(self.x, v2.x)
            && comparing_floating_number(self.y, v2.y)
            && comparing_floating_number(self.z, v2.z)
    }
    fn add_vector(&self, other: &Self) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    fn add_point(&self, p: Point) -> Point {
        Point::new(self.x + p.x, self.y + p.y, self.z + p.z)
    }
    fn subtrac_vector(&self, other: &Self) -> Self {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    fn multi_scalar(&self, scalar: f64) -> Self {
        Vector::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
    fn div_scalar(&self, scalar: f64) -> Self {
        Vector::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
    fn lenght(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
    pub fn normalization(&self) -> Self {
        Vector::new(
            self.x / self.lenght(),
            self.y / self.lenght(),
            self.z / self.lenght(),
        )
    }
    pub fn dot_product(&self, other: Self) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    pub fn cross_product(&self, other: Self) -> Self {
        Vector::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
    pub fn reflect(self, normal: Vector) -> Vector {
        self - normal * 2.0 * self.dot_product(normal)
    }
}

//Arithmetic Operators Overload
impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        self.add_vector(&rhs)
    }
}
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        self.subtrac_vector(&rhs)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        self.multi_scalar(rhs)
    }
}
impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self.div_scalar(rhs)
    }
}

//Boolean Operators Overload
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.equal_vector(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_does_stored_values() {
        let v = Vector::new(4.3, -4.2, 3.1);

        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.0);
    }

    #[test]
    fn vector_equal_vector() {
        let v = Vector::new(4.3, -4.2, 3.1);
        let v2 = Vector::new(4.3, -4.200001, 3.1);

        assert!(v.equal_vector(&v2));
        assert!(v == v2);
    }

    #[test]
    fn vector_add_vector() {
        let v = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(
            v + v2,
            Vector {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 0.0
            }
        );
    }

    #[test]
    fn vector_add_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(p + v, Point::new(1.0, 1.0, 6.0))
    }
    #[test]
    fn vector_sub_vector() {
        let v = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(v - v2, Vector::new(-2.0, -4.0, -6.0))
    }
    #[test]
    fn scalar_multi_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(v * 3.5, Vector::new(3.5, -7.0, 10.5));
        assert_eq!(v * 0.5, Vector::new(0.5, -1.0, 1.5));
    }
    #[test]
    fn scalar_div_point() {
        let p = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(p / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn neg_vector() {
        let v = Vector::new(3.0, 2.0, 1.0);

        assert_eq!(-v, Vector::new(-3.0, -2.0, -1.0))
    }
    #[test]
    fn magnitude() {
        let v = Vector::new(0.0, 0.0, 1.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        let h: f64 = 14.0;
        assert_eq!(v.lenght(), 1.0);
        assert_eq!(v2.lenght(), h.sqrt());
    }
    #[test]
    fn normalizing_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.normalization(), Vector::new(0.26726, 0.53452, 0.80178))
    }
    #[test]
    fn vector_dot_product() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let v1 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v.dot_product(v1), 20.0);
    }
    #[test]
    fn vector_cross_product() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let v1 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v.cross_product(v1), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v1.cross_product(v), Vector::new(1.0, -2.0, 1.0));
    }
    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
    #[test]
    fn reflecting_a_vector_approaching_at_45_grades() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);

        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }
}
