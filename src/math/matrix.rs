#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    data: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        Matrix4 { data }
    }
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self {
            data: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn get_matrix() {
        let data = [
            [1.0, 2.0, 3.0, 2.0],
            [4.0, 5.0, 6.0, 5.0],
            [7.0, 9.0, 5.0, 7.0],
            [1.0, 2.0, 3.0, 3.0],
        ];

        let matrix = Matrix4::new(data);

        assert_eq!(matrix.get(0, 0), 1.0);
    }
    #[test]
    fn set_value_in_matrix() {
        let data = [
            [1.0, 2.0, 3.0, 2.0],
            [4.0, 5.0, 6.0, 5.0],
            [7.0, 9.0, 5.0, 7.0],
            [1.0, 2.0, 3.0, 3.0],
        ];

        let mut matrix = Matrix4::new(data);

        matrix.set(0, 0, 0.0);

        assert_eq!(matrix.get(0, 0), 0.0);
    }
    #[test]
    fn equal_matrix() {
        let data = [
            [1.0, 2.0, 3.0, 2.0],
            [4.0, 5.0, 6.0, 5.0],
            [7.0, 9.0, 5.0, 7.0],
            [1.0, 2.0, 3.0, 3.0],
        ];

        let matrix1 = Matrix4::new(data);
        let matrix2 = Matrix4::new(data);

        assert_eq!(matrix1, matrix2);
    }
    #[test]
    fn equality_with_different_matrices() {
        let data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];

        let data2 = [
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ];

        let matrix1 = Matrix4::new(data);
        let matrix2 = Matrix4::new(data2);

        assert!(matrix2 != matrix1);
    }
}
