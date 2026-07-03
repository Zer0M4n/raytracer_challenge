use crate::math::point::Point;
use crate::math::vector::{self, Vector};
use crate::utils::{self, comparing_floating_number, radians};
use core::f64;
use std::cmp::PartialEq;
use std::convert::identity;
use std::ops::Mul;
use std::result;

const PI_64: f64 = std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![0.0; rows * cols];
        Matrix { rows, cols, data }
    }
    pub fn from_vec(rows: usize, cols: usize, value: Vec<f64>) -> Result<Self, String> {
        if value.len() != (rows * cols) {
            return Err(format!(
                "Matrix size mismatch: expected {} elements, got {}",
                rows * cols,
                value.len()
            ));
        }
        Ok(Matrix {
            rows,
            cols,
            data: value,
        })
    }
    pub fn get(&self, rows: usize, cols: usize) -> f64 {
        let index = rows * self.cols + cols;
        self.data[index]
    }
    pub fn set(&mut self, rows: usize, cols: usize, value: f64) {
        let index = rows * self.cols + cols;
        self.data[index] = value;
    }
    pub fn identity(value: usize) -> Self {
        let mut data = vec![0.0; value * value];
        let mut counter = 0;

        for x in 0..value {
            counter = x;
            let index = x * value + counter;

            data[index] = 1.0;
        }

        Matrix::from_vec(value, value, data).unwrap()
    }
    pub fn transpose(&mut self) {
        let copy_m = self.data.clone();

        for r in 0..self.rows {
            for c in 0..self.cols {
                //index for row-major order
                let index_rmo = r * self.cols + c;

                //index for colum major ordr
                let index_cmo = r + c * self.rows;

                self.data[index_rmo] = copy_m[index_cmo];
            }
        }
    }
    pub fn determinant(&self) -> f64 {
        let n = self.rows;

        if n == 1 {
            return self.get(0, 0);
        }

        if n == 2 {
            return self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0);
        }

        let mut det = 0.0;

        for j in 0..n {
            let minor = Matrix::delete_row_column(self, 0, j);

            let sign = if j % 2 == 0 { 1.0 } else { -1.0 };

            det += sign * self.get(0, j) * minor.determinant();
        }

        det
    }
    pub fn inverse(&self) -> Result<Matrix, String> {
        let det = self.determinant();

        if det == 0.0 {
            return Err("This matrix has no inverse".to_string());
        }

        let mut cof = Matrix::new(self.rows, self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                cof.set(r, c, self.cofactore(r, c));
            }
        }

        cof.transpose();

        Ok(cof * (1.0 / det))
    }
    pub fn traslation(x: f64, y: f64, z: f64) -> Self {
        let mut t = Matrix::identity(4);
        t.set(0, 3, x);
        t.set(1, 3, y);
        t.set(2, 3, z);

        t
    }
    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut result = Matrix::identity(4);

        result.set(0, 0, x);
        result.set(1, 1, y);
        result.set(2, 2, z);

        result
    }
    pub fn rotation_x(radians: f64) -> Self {
        let mut result = Matrix::identity(4);

        result.set(1, 1, radians.cos());
        result.set(1, 2, -radians.sin());
        result.set(2, 1, radians.sin());
        result.set(2, 2, radians.cos());

        result
    }
    pub fn rotation_y(radians: f64) -> Self {
        let mut result = Matrix::identity(4);

        result.set(0, 0, radians.cos());
        result.set(0, 2, radians.sin());
        result.set(2, 0, -radians.sin());
        result.set(2, 2, radians.cos());

        result
    }
    pub fn rotation_z(radians: f64) -> Self {
        let mut result = Matrix::identity(4);

        result.set(0, 0, radians.cos());
        result.set(0, 1, -radians.sin());
        result.set(1, 0, radians.sin());
        result.set(1, 1, radians.cos());

        result
    }
    pub fn shearing(x: f64, x2: f64, y: f64, y2: f64, z: f64, z2: f64) -> Self {
        let mut result = Matrix::identity(4);

        result.set(0, 1, x);
        result.set(0, 2, x2);

        result.set(1, 0, y);
        result.set(1, 2, y2);

        result.set(2, 0, z);
        result.set(2, 1, z2);

        result
    }
    fn cofactore(&self, r: usize, c: usize) -> f64 {
        let submatrix = self.delete_row_column(r, c);

        let menor = submatrix.determinant();

        let signo = if (r + c) % 2 == 0 { 1.0 } else { -1.0 };

        menor * signo
    }
    fn delete_row_column(&self, row: usize, col: usize) -> Matrix {
        let mut s = Matrix::new(self.rows - 1, self.cols - 1);

        let mut new_r = 0;

        for r in 0..self.rows {
            if r == row {
                continue;
            }

            let mut new_c = 0;

            for c in 0..self.cols {
                if c == col {
                    continue;
                }

                s.set(new_r, new_c, self.get(r, c));

                new_c += 1;
            }

            new_r += 1;
        }

        s
    }
    fn multi(&self, rhs: &Matrix) -> Result<Matrix, String> {
        if self.cols != rhs.rows {
            return Err(
            "The number of columns of the first matrix must equal the number of rows of the second matrix."
                .to_string(),
            );
        }

        let mut result = Matrix::new(self.rows, rhs.cols);

        for r in 0..self.rows {
            for c in 0..rhs.cols {
                let mut sum = 0.0;

                for k in 0..self.cols {
                    sum += self.get(r, k) * rhs.get(k, c);
                }

                result.set(r, c, sum);
            }
        }

        Ok(result)
    }
    fn multi_scalar(&self, value: f64) -> Matrix {
        let mut nuevos_datos = self.data.clone();

        for x in nuevos_datos.iter_mut() {
            *x *= value;
        }
        Matrix::from_vec(self.rows, self.cols, nuevos_datos).unwrap()
    }
    fn equal(&self, value: &Matrix) -> bool {
        if self.rows != value.rows || self.cols != value.cols {
            return false;
        }

        for i in 0..self.data.len() {
            if !comparing_floating_number(self.data[i], value.data[i]) {
                return false;
            }
        }

        true
    }

    //fluent api
    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        self = (Matrix::traslation(x, y, z) * self).unwrap();
        self
    }

    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self = (Matrix::scaling(x, y, z) * self).unwrap();
        self
    }

    pub fn rotate_x(mut self, radians: f64) -> Self {
        self = (Matrix::rotation_x(radians) * self).unwrap();
        self
    }

    pub fn rotate_y(mut self, radians: f64) -> Self {
        self = (Matrix::rotation_y(radians) * self).unwrap();
        self
    }

    pub fn rotate_z(mut self, radians: f64) -> Self {
        self = (Matrix::rotation_z(radians) * self).unwrap();
        self
    }
    pub fn shear(mut self, x: f64, x2: f64, y: f64, y2: f64, z: f64, z2: f64) -> Self {
        self = (Matrix::shearing(x, x2, y, y2, z, z2) * self).unwrap();
        self
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Result<Matrix, String>;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        self.multi(rhs)
    }
}
impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        self.multi_scalar(rhs)
    }
}
impl Mul<Point> for &Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(
            self.get(0, 0) * rhs.x
                + self.get(0, 1) * rhs.y
                + self.get(0, 2) * rhs.z
                + self.get(0, 3),
            self.get(1, 0) * rhs.x
                + self.get(1, 1) * rhs.y
                + self.get(1, 2) * rhs.z
                + self.get(1, 3),
            self.get(2, 0) * rhs.x
                + self.get(2, 1) * rhs.y
                + self.get(2, 2) * rhs.z
                + self.get(2, 3),
        )
    }
}
impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(
            self.get(0, 0) * rhs.x + self.get(0, 1) * rhs.y + self.get(0, 2) * rhs.z,
            self.get(1, 0) * rhs.x + self.get(1, 1) * rhs.y + self.get(1, 2) * rhs.z,
            self.get(2, 0) * rhs.x + self.get(2, 1) * rhs.y + self.get(2, 2) * rhs.z,
        )
    }
}
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}
impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        (&self).mul(rhs)
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        (&self).mul(rhs)
    }
}

impl Mul for Matrix {
    type Output = Result<Matrix, String>;

    fn mul(self, rhs: Matrix) -> Self::Output {
        (&self).mul(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_values() {
        let mut m = Matrix::new(4, 4);

        m.set(1, 1, 1.0);

        assert_eq!(m.data[0], 0.0);
        assert_eq!(m.data[1 * 4 + 1], 1.0);
    }
    #[test]
    fn get_values() {
        let m = Matrix::new(4, 4);

        assert_eq!(m.get(0, 0), 0.0);
    }
    #[test]
    fn equality_with_identical_matrices() {
        let m = Matrix::new(2, 2);
        let m2 = Matrix::new(2, 2);

        assert_eq!(m, m2);
        assert!(m == m2);
    }
    #[test]
    fn equality_with_different_matrices() {
        let m = Matrix::new(2, 2);
        let mut m2 = Matrix::new(2, 2);

        assert!(m == m2);
        m2.data[0] = 1.0;
        assert!(m != m2);
    }
    #[test]
    fn matrix_multiply() {
        let v1 = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let v2 = vec![
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ];
        let v3 = vec![
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
            46.0, 42.0,
        ];
        let m1 = Matrix::from_vec(4, 4, v1).unwrap();
        let m2 = Matrix::from_vec(4, 4, v2).unwrap();
        let m3 = Matrix::from_vec(4, 4, v3).unwrap();

        let result = (m1 * m2).unwrap();
        assert_eq!(result, m3);
    }
    #[test]
    fn multiplied_by_a_tuple() {
        let v1 = vec![
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let tuple = vec![1.0, 2.0, 3.0, 1.0];
        let r = vec![18.0, 24.0, 33.0, 1.0];
        let re = Matrix::from_vec(4, 1, r).unwrap();
        let m = Matrix::from_vec(4, 4, v1).unwrap();
        let matrix_tuple = Matrix::from_vec(4, 1, tuple).unwrap();
        assert_eq!((m * matrix_tuple).unwrap(), re);
    }
    #[test]
    fn set_matrix_identity() {
        let identity = Matrix::identity(4);

        assert_eq!(identity.get(0, 0), 1.0);
        assert_eq!(identity.get(1, 1), 1.0);
        assert_eq!(identity.get(2, 2), 1.0);
        assert_eq!(identity.get(3, 3), 1.0);
    }
    #[test]
    fn set_transpose() {
        let v1 = vec![
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ];

        let v2 = vec![
            0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
        ];

        let mut m1 = Matrix::from_vec(4, 4, v1).unwrap();
        let m2 = Matrix::from_vec(4, 4, v2).unwrap();
        m1.transpose();
        assert_eq!(m1, m2);
    }
    #[test]
    fn determinants_matrix() {
        let data3x3 = vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0];
        let m3x3 = Matrix::from_vec(3, 3, data3x3).unwrap();
        assert_eq!(m3x3.determinant(), -93.0)
    }
    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let data3x3 = vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0];
        let m = Matrix::from_vec(3, 3, data3x3).unwrap();

        let s = m.delete_row_column(1, 0);

        assert_eq!(s.determinant(), 25.0)
    }
    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let mut data = vec![
            8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
        ];
        let data_inverse = vec![
            -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
            0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
        ];

        let m = Matrix::from_vec(4, 4, data).unwrap();

        let m_inverse = Matrix::from_vec(4, 4, data_inverse).unwrap();
        let inverse = m.inverse().unwrap();
        assert!(inverse.equal(&m_inverse));
        assert_eq!(m.inverse().unwrap(), m_inverse);
    }
    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix::traslation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Point::new(2.0, 1.0, 7.0))
    }
    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::traslation(5.0, -3.0, 2.0);
        let inv = Matrix::inverse(&transform).unwrap();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0))
    }
    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::traslation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v)
    }
    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Vector::new(-8.0, 18.0, 32.0))
    }
    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = Matrix::inverse(&transform).unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Vector::new(-2.0, 2.0, 2.0))
    }
    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(-2.0, 3.0, 4.0))
    }
    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarte = Matrix::rotation_x(PI_64 / 4.0);
        let inv = half_quarte.inverse().unwrap();

        assert_eq!(
            inv * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        )
    }
    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI_64 / 4.0);
        let full_quarter = Matrix::rotation_y(PI_64 / 2.0);

        assert_eq!(
            half_quarter * p,
            Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    }
    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI_64 / 4.0);
        let full_quarter = Matrix::rotation_z(PI_64 / 2.0);

        assert_eq!(
            half_quarter * p,
            Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }
    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(5.0, 3.0, 4.0))
    }
    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(6.0, 3.0, 4.0))
    }
    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 5.0, 4.0))
    }
    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 7.0, 4.0))
    }
    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 3.0, 6.0))
    }
    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 3.0, 7.0))
    }
    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI_64 / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::traslation(10.0, 5.0, 7.0);

        let p2 = &a * p;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));

        let p3 = &b * p2;
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));

        let p4 = &c * p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));

        let t = (&c * &b).unwrap();
        let t = (&t * &a).unwrap();

        assert_eq!(&t * p, Point::new(15.0, 0.0, 7.0));
    }
    #[test]
    fn fluent_api() {
        let transform = Matrix::identity(4)
            .rotate_x(PI_64 / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        let p = Point::new(1.0, 0.0, 1.0);
        assert_eq!(transform * p, Point::new(15.0, 0.0, 7.0));
    }
}
