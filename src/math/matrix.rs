use std::cmp::PartialEq;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]

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

    fn multi(&self, rhs: Matrix) -> Result<Matrix, String> {
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
}

impl Mul for Matrix {
    type Output = Result<Matrix, String>;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self.multi(rhs)
    }
}

#[cfg(test)]
mod tests {
    use core::prelude::v1;

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
        let v1 = vec![1.0 , 2.0 , 3.0 , 4.0 ,
                                2.0 , 4.0 , 4.0 , 2.0 ,
                                8.0 , 6.0 , 4.0 , 1.0 ,
                                0.0 , 0.0 , 0.0 , 1.0 ];
        let tuple = vec![1.0,2.0,3.0,1.0];
        let r = vec![18.0,24.0,33.0,1.0];
        let re = Matrix::from_vec(4, 1, r).unwrap();
        let m = Matrix::from_vec(4, 4, v1).unwrap();
        let matrix_tuple = Matrix::from_vec(4, 1, tuple).unwrap();
        assert_eq!((m * matrix_tuple).unwrap() , re);
    }
}
