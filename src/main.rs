use std::ops;
use std::cmp;

pub struct  Tuple{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    fn point(x: f64 ,y: f64 ,z: f64 ) -> Self {
        Self { x, y, z, w:1.0 }
    }
    fn vector(x: f64 ,y: f64 ,z: f64 ) -> Self {
        Self { x, y, z, w:0.0 }
    }
    fn is_point(&self) -> bool{
        self.w == 1.0
    }
    fn is_vector(&self) -> bool{
        self.w == 0.0 
    }
    fn equal_point(left: f64, right: f64) -> bool {
        const EPSILON: f64 = 0.00001; 
        (left - right).abs() < EPSILON 
    }
    fn is_equal(&self, other: &Tuple) -> bool {
        Self::equal_point(self.x, other.x)
            && Self::equal_point(self.y, other.y)
            && Self::equal_point(self.z, other.z)
            && Self::equal_point(self.w, other.w)
    }
}
impl cmp::PartialEq for Tuple{
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use  super::*;

    #[test]
    fn point_does_stored_values() {
        let p = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
    }
    #[test]
    fn point_is_point(){
        let p = Tuple::point(4.3, -4.2, 3.1);

        assert!(p.is_point());
    }
    #[test]
    fn vector_does_stored_values(){
        let v = Tuple::vector(4.3,-4.2,3.1);

        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
    }
    #[test]
    fn vecor_is_vector(){
        let v = Tuple::vector(4.3,-4.2,3.1);

        assert!(v.is_vector());
    }
    #[test]
    fn vector_equal_vector(){
        let v = Tuple::vector(4.3,-4.2,3.1);
        
        
    }



}
