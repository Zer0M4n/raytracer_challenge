use std::ops::ControlFlow::Break;

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
        let index = rows * cols + cols;
        self.data[index]
    }
    pub fn set(&mut self, rows: usize, cols: usize, value: f64) {
        let index = rows * cols + cols;
        self.data[index] = value;
    }

    fn multi(&mut self, value: Matrix) -> Result<Self, String> {
        if self.cols != value.rows {
            return Err(format!(
                "The number of columns of the first matrix must equal the number of rows of the second matrix."
            ));
        }

        let mut m = Matrix::new(self.rows, value.cols);
        for r in 0..self.rows {
            for c in 0..value.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    let data = self.get(r, c) * value.get(r, c);
                    sum += self.get(r, k) * value.get(k, c);
                }
                m.set(r, c, sum);
            }
        }

        Ok(m)
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
        assert_eq!(m.data[1 * 1 + 1], 1.0);
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
}
