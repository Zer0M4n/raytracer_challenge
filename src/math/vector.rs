use crate::math::utils::comparing_floating_number;

//The logic vector , w is 0.0 for math facilities
#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vector {
    pub fn new(x:f64 ,y:f64,z:f64) -> Self {
        Self { x, y, z, w:0.0 }
    }
    fn equal_vector(&self,v2: Vector ) -> bool {
        comparing_floating_number(self.x, v2.x) &&
        comparing_floating_number(self.y, v2.y) &&
        comparing_floating_number(self.z, v2.z)  
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
        let v2 = Vector::new(4.3, -4.2000001, 3.1);
        
        assert!(v.equal_vector(v2));
    }

}