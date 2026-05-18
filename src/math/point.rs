use crate::math::utils::comparing_floating_number;


//The logic point , w is 1.0 for math facilities
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Point {
    fn new(x:f64 ,y:f64,z:f64) -> Self {
        Self { x, y, z, w:1.0 }
    }

    fn equal_point(&self,v2: Point ) -> bool {
        comparing_floating_number(self.x, v2.x) &&
        comparing_floating_number(self.y, v2.y) &&
        comparing_floating_number(self.z, v2.z)  
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
        let p2 = Point::new(4.3, -4.2, 3.1);

        assert!(p.equal_point(p2));

    }

}