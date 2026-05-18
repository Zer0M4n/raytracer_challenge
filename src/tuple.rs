#[derive(Debug, Copy, Clone, PartialEq)]

impl Tuple {
    
    pub struct  Tuple{
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }

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
    // fn equal_point(left: f64, right: f64) -> bool {
    //     const EPSILON: f64 = 0.00001; 
    //     (left - right).abs() < EPSILON 
    // }
    // fn is_equal(&self, other: &Tuple) -> bool {
    //     Self::equal_point(self.x, other.x)
    //         && Self::equal_point(self.y, other.y)
    //         && Self::equal_point(self.z, other.z)
    //         && Self::equal_point(self.w, other.w)
    // }

    //Arithmetics Operatios
}
