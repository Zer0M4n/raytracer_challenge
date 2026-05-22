use crate::math::point::Point;
use crate::math::utils::comparing_floating_number;
use std::cmp::PartialEq;
use std::ops::{Add, Sub, Neg};
//The logic vector , w is 0.0 for math facilities
#[derive(Debug)]
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
    fn add_point(&self, p: Point) -> Point{
        Point::new(
            self.x +p.x,
            self.y +p.y,
            self.z +p.z
        )
    }
    fn subtrac_vector(&self, other: &Self) -> Self {
        Vector::new(
            self.x - other.x, 
            self.y - other.y, 
            self.z - other.z
        )

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

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtrac_vector(&rhs)
    }
    
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(
            -self.x, 
            -self.y, 
            -self.z
        )
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

        assert_eq!(
            p + v ,Point::new(1.0, 1.0, 6.0)
        )    
    }
    #[test]
    fn vector_sub_vector() {
        let v = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(
            v - v2,
            Vector::new(-2.0, -4.0, -6.0)
        )
    }
    #[test]
    fn neg_vector() {
        let v = Vector::new(3.0, 2.0, 1.0);

        assert_eq!(-v, Vector::new(-3.0, -2.0, -1.0))
        
    }


}
