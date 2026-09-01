use crate::math::{matrix::Matrix, point::Point, vector::Vector};

const PI_64: f64 = std::f64::consts::PI;
const EPSILON: f64 = 0.0001;
// This fuctions exist for comparing float number with EPSILON for round of Error make two number that should
// be  equivalent for instead be slighty different
pub fn comparing_floating_number(x: f64, y: f64) -> bool {
    (x - y).abs() < EPSILON
}
pub fn radians(deg: f64) -> f64 {
    (deg / 180.0) * PI_64
}

//view transformation
pub fn view_transformation(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).normalization();
    let upn = up.normalization();
    let left = forward.cross_product(upn);
    let true_up = left.cross_product(forward);
    let v_orientation = vec![
        left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0, -forward.x, -forward.y,
        -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    let mut orientation = Matrix::from_vec(4, 4, v_orientation).unwrap();

    let result = orientation * Matrix::traslation(-from.x, -from.y, -from.z);
    result.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, Matrix::identity(4))
    }
    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0))
    }
    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transformation(from, to, up);

        assert_eq!(t, Matrix::traslation(0.0, 0.0, -8.0))
    }
    //     #[test]
    //     fn an_arbitrary_view_transformation() {
    //         let from = Point::new(1.0, 3.0, 2.0);
    //         let to = Point::new(4.0, -2.0, 8.0);
    //         let up = Vector::new(1.0, 1.0, 0.0);

    //         let t = view_transformation(from, to, up);
    //         let v_result = vec![
    //              -0.50709 , 0.50709 , 0.67612 , -2.36643 ,
    //  0.76772 , 0.60609 , 0.12122 , -2.82843 ,
    //  -0.35857 , 0.59761 , -0.71714 , 0.00000 ,
    //  0.00000 , 0.00000 , 0.00000 , 1.00000
    //         ];
    //         assert_eq!(t, Matrix::from_vec(4, 4, v_result).unwrap())
    //     }
}
