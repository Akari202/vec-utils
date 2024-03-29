/// Functions for working with 2x2 matrices
pub mod matrix2x2{
    /// Calculate the determinant of a 2x2 matrix
    pub fn determinant(matrix: &[[f64; 2]; 2]) -> f64 {
        matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
    }
}

/// Functions for working with 3x3 matrices
pub mod matrix3x3 {
    /// Calculate the determinant of a 3x3 matrix
    pub fn determinant(matrix: &[[f64; 3]; 3]) -> f64 {
        matrix[0][0] * matrix[1][1] * matrix[2][2] +
            matrix[0][1] * matrix[1][2] * matrix[2][0] +
            matrix[0][2] * matrix[1][0] * matrix[2][1] -
            matrix[0][2] * matrix[1][1] * matrix[2][0] -
            matrix[0][1] * matrix[1][0] * matrix[2][2] -
            matrix[0][0] * matrix[1][2] * matrix[2][1]
    }

    /// Calculate the minor of a 3x3 matrix given a row and column index
    pub fn minor(matrix: &[[f64; 3]; 3], row: usize, col: usize) -> f64 {
        let mut minor = [[0.0; 2]; 2];
        for i in 0..3 {
            for j in 0..3 {
                if i != row && j != col {
                    let mut m = i;
                    let mut n = j;
                    if i > row {
                        m -= 1;
                    }
                    if j > col {
                        n -= 1;
                    }
                    minor[m][n] = matrix[i][j];
                }
            }
        }
        super::matrix2x2::determinant(&minor)
    }

    /// Calculate the cofactor of a 3x3 matrix given a row and column index
    pub fn cofactor(matrix: &[[f64; 3]; 3], row: usize, col: usize) -> f64 {
        let minor = minor(matrix, row, col);
        -1.0_f64.powf((row + col + 2) as f64) * minor
    }
}

/// Functions for working with 4x4 matrices
pub mod matrix4x4 {
    /// Calculate the determinant of a 4x4 matrix
    pub fn determinant(matrix: &[[f64; 4]; 4]) -> f64 {
        matrix[0][0] * matrix[1][1] * matrix[2][2] * matrix[3][3] +
            matrix[0][0] * matrix[1][2] * matrix[2][3] * matrix[3][1] +
            matrix[0][0] * matrix[1][3] * matrix[2][1] * matrix[3][2] +
            matrix[0][1] * matrix[1][0] * matrix[2][3] * matrix[3][2] +
            matrix[0][1] * matrix[1][2] * matrix[2][0] * matrix[3][3] +
            matrix[0][1] * matrix[1][3] * matrix[2][2] * matrix[3][0] +
            matrix[0][2] * matrix[1][0] * matrix[2][1] * matrix[3][3] +
            matrix[0][2] * matrix[1][1] * matrix[2][3] * matrix[3][0] +
            matrix[0][2] * matrix[1][3] * matrix[2][0] * matrix[3][1] +
            matrix[0][3] * matrix[1][0] * matrix[2][2] * matrix[3][1] +
            matrix[0][3] * matrix[1][1] * matrix[2][0] * matrix[3][2] +
            matrix[0][3] * matrix[1][2] * matrix[2][1] * matrix[3][0] -
            matrix[0][0] * matrix[1][1] * matrix[2][3] * matrix[3][2] -
            matrix[0][0] * matrix[1][2] * matrix[2][1] * matrix[3][3] -
            matrix[0][0] * matrix[1][3] * matrix[2][2] * matrix[3][1] -
            matrix[0][1] * matrix[1][0] * matrix[2][2] * matrix[3][3] -
            matrix[0][1] * matrix[1][2] * matrix[2][3] * matrix[3][0] -
            matrix[0][1] * matrix[1][3] * matrix[2][0] * matrix[3][2] -
            matrix[0][2] * matrix[1][0] * matrix[2][3] * matrix[3][1] -
            matrix[0][2] * matrix[1][1] * matrix[2][0] * matrix[3][3] -
            matrix[0][2] * matrix[1][3] * matrix[2][1] * matrix[3][0] -
            matrix[0][3] * matrix[1][0] * matrix[2][1] * matrix[3][2] -
            matrix[0][3] * matrix[1][1] * matrix[2][2] * matrix[3][0] -
            matrix[0][3] * matrix[1][2] * matrix[2][0] * matrix[3][1]
    }

    /// Calculate the minor of a 4x4 matrix given a row and column index
    pub fn minor(matrix: &[[f64; 4]; 4], row: usize, col: usize) -> f64 {
        let mut minor = [[0.0; 3]; 3];
        for i in 0..4 {
            for j in 0..4 {
                if i != row && j != col {
                    let mut m = i;
                    let mut n = j;
                    if i > row {
                        m -= 1;
                    }
                    if j > col {
                        n -= 1;
                    }
                    minor[m][n] = matrix[i][j];
                }
            }
        }
        super::matrix3x3::determinant(&minor)
    }

    /// Calculate the cofactor of a 4x4 matrix given a row and column index
    pub fn cofactor(matrix: &[[f64; 4]; 4], row: usize, col: usize) -> f64 {
        let minor = minor(matrix, row, col);
        -1.0_f64.powf((row + col + 2) as f64) * minor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix2x2_determinant() {
        let matrix = [
            [1.0, 2.0],
            [3.0, 4.0]
        ];
        assert_eq!(matrix2x2::determinant(&matrix), -2.0);
    }

    #[test]
    fn test_matrix3x3_determinant() {
        let matrix = [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0]
        ];
        assert_eq!(matrix3x3::determinant(&matrix), 0.0);
    }

    #[test]
    fn test_matrix3x3_minor() {
        let matrix = [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0]
        ];
        assert_eq!(matrix3x3::minor(&matrix, 0, 0), -3.0);
    }

    #[test]
    fn test_matrix3x3_cofactor() {
        let matrix = [
            [1.0, 4.0, 7.0],
            [3.0, 0.0, 5.0],
            [-1.0, 9.0, 11.0]
        ];
        assert_eq!(matrix3x3::cofactor(&matrix, 1, 2), -13.0);
    }

    #[test]
    fn test_matrix4x4_determinant() {
        let matrix = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0]
        ];
        assert_eq!(matrix4x4::determinant(&matrix), 0.0);
    }

    #[test]
    fn test_matrix4x4_minor() {
        let matrix = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0]
        ];
        assert_eq!(matrix4x4::minor(&matrix, 0, 0), 0.0);
    }

    #[test]
    fn test_matrix4x4_cofactor() {
        let matrix = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0]
        ];
        assert_eq!(matrix4x4::cofactor(&matrix, 0, 0), 0.0);
    }
}
