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
    pub fn get(&self, rows: usize, cols: usize) -> f64 {
        let index = rows * cols + cols;
        self.data[index]
    }
    pub fn set(&mut self, rows: usize, cols: usize, value: f64) {
        let index = rows * cols + cols;
        self.data[index] = value;
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
