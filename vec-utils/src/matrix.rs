use core::ops::{Index, IndexMut, Mul};
#[cfg(feature = "std")]
use std::vec::Vec;

use matrixmultiply::dgemm;

/// A generic 2d matrix of width R and height C
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize>
where
    [f64; R * C]: Sized
{
    pub(crate) values: [f64; R * C]
}

/// An alias for 2x2 matracies
pub type Matrix2x2 = Matrix<2, 2>;
/// An alias for 3x3 matracies
pub type Matrix3x3 = Matrix<3, 3>;

impl<const R: usize, const C: usize> Matrix<R, C>
where
    [f64; R * C]: Sized
{
    /// Create a matrix from nested vectors.
    /// # Panics
    /// A misshapen vector, i.e. one that's not of length C or that doesnt contain vectors of exclusively length R
    #[cfg(feature = "std")]
    pub fn from_nested_vec(values: Vec<Vec<f64>>) -> Self {
        let flattened: Vec<f64> = values.into_iter().flatten().collect();
        let values: [f64; R * C] = flattened
            .try_into()
            .expect("Input dimensions do not match Matrix size R * C");
        Self { values }
    }

    /// Create a matrix from nested arrays.
    pub fn from_nested_arr(values: [[f64; C]; R]) -> Self {
        // Safety: [[f64; C]; R] and [f64; R * C] have the exact same memory layout
        let flat_values = unsafe {
            let ptr = (&raw const values).cast::<[f64; R * C]>();
            core::ptr::read(ptr)
        };

        let _ = values;

        Self {
            values: flat_values
        }
    }

    /// Create a matrix filled with zeros
    pub fn zeros() -> Self {
        Self {
            values: [0.0; R * C]
        }
    }

    /// Create a matrix filled with ones
    pub fn ones() -> Self {
        Self {
            values: [1.0; R * C]
        }
    }

    /// Determines if the matrix is square
    pub fn is_square(&self) -> bool {
        R == C
    }

    /// Counts the number of nonzero values
    pub fn count_nonzero(&self) -> usize {
        self.values
            .iter()
            .fold(0, |acc, i| if *i == 0.0 { acc } else { acc + 1 })
    }

    /// Returns the diagonal elements
    #[cfg(feature = "std")]
    pub fn diagonals(&self) -> Vec<f64> {
        let min_dimm = R.min(C);
        (0..min_dimm).map(|i| self.values[i + i * C]).collect()
    }

    /// Checks if the matrix is upper triangluar
    /// This does not check if its strictly upper triangluar
    pub fn is_upper_triangular(&self) -> bool {
        todo!()
    }

    /// Checks if the matrix is a diagonal matrix
    pub fn is_diagonal(&self) -> bool {
        todo!()
    }

    /// Iterates over the matrix with enumerated position values
    pub fn iter_indexed(&self) -> impl Iterator<Item = ((usize, usize), &f64)> {
        self.values.iter().enumerate().map(|(idx, val)| {
            let r = idx / C;
            let c = idx % C;
            ((r, c), val)
        })
    }

    /// Iterates over the matrix
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.values.iter()
    }

    /// Mutably iterates over the matrix
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
        self.values.iter_mut()
    }

    /// Calculates the determinant of the matrix
    pub fn determinant(&self) -> f64 {
        if self.count_nonzero() == 0 {
            0.0
        } else {
            todo!()
        }
    }
}

impl<const R: usize, const C: usize> Index<[usize; 2]> for Matrix<R, C>
where
    [f64; R * C]: Sized
{
    type Output = f64;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        let [row, col] = idx;
        assert!(
            row < R && col < C,
            "Index [{row}, {col}] is out of bounds for matrix of shape [{R}, {C}]"
        );
        &self.values[row * C + col]
    }
}

impl<const R: usize, const C: usize> IndexMut<[usize; 2]> for Matrix<R, C>
where
    [f64; R * C]: Sized
{
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        let [row, col] = idx;
        assert!(
            row < R && col < C,
            "Index [{row}, {col}] is out of bounds for matrix of shape [{R}, {C}]"
        );
        &mut self.values[row * C + col]
    }
}

impl<const R: usize, const C: usize, const U: usize> Mul<Matrix<U, C>> for Matrix<R, U>
where
    [f64; R * C]: Sized,
    [f64; R * U]: Sized,
    [f64; U * C]: Sized
{
    type Output = Matrix<R, C>;

    fn mul(self, rhs: Matrix<U, C>) -> Self::Output {
        let mut result = Matrix::<R, C>::zeros();
        unsafe {
            dgemm(
                R,
                U,
                C,
                1.0,
                self.values.as_ptr(),
                U.cast_signed(),
                1,
                rhs.values.as_ptr(),
                C.cast_signed(),
                1,
                0.0,
                result.values.as_mut_ptr(),
                C.cast_signed(),
                1
            );
        }
        result
    }
}

#[cfg(test)]
mod tests_2x2 {
    use assert_float_eq::assert_f64_near;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_initialization() {
        let arr = [[1.0, 2.0], [3.0, 4.0]];
        let mat = Matrix2x2::from_nested_arr(arr);
        assert_f64_near!(mat[[0, 0]], 1.0);
        assert_f64_near!(mat[[1, 1]], 4.0);

        #[cfg(feature = "std")]
        {
            let vec_data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
            let mat_vec = Matrix2x2::from_nested_vec(vec_data);
            assert_f64_near!(mat_vec[[1, 0]], 3.0);
        }

        let z = Matrix2x2::zeros();
        let o = Matrix2x2::ones();
        assert!(z.iter().all(|&v| v == 0.0));
        assert!(o.iter().all(|&v| (v - 1.0).abs() < f64::EPSILON));
    }

    #[cfg(feature = "std")]
    #[test]
    #[should_panic(expected = "Input dimensions do not match Matrix size R * C")]
    fn test_from_nested_vec_panic() {
        let bad_vec = vec![vec![1.0], vec![2.0, 3.0]];
        let _ = Matrix2x2::from_nested_vec(bad_vec);
    }

    #[test]
    fn test_properties() {
        let square = Matrix2x2::zeros();
        let non_square = Matrix::<2, 3>::zeros();

        assert!(square.is_square());
        assert!(!non_square.is_square());

        let mut mat = Matrix2x2::zeros();
        mat[[0, 0]] = 1.0;
        mat[[1, 1]] = 5.0;
        assert_eq!(mat.count_nonzero(), 2);
    }

    #[test]
    fn test_matrix_structure() {
        let ut = Matrix2x2::from_nested_arr([[1.0, 2.0], [0.0, 3.0]]);
        assert!(ut.is_upper_triangular());

        #[cfg(feature = "std")]
        {
            let diag = Matrix2x2::from_nested_arr([[1.0, 0.0], [0.0, 3.0]]);
            assert!(diag.is_diagonal());
            assert_eq!(diag.diagonals(), vec![1.0, 3.0]);
        }

        assert!(!ut.is_diagonal());
    }

    #[test]
    fn test_iterators() {
        let mut mat = Matrix2x2::from_nested_arr([[1.0, 2.0], [3.0, 4.0]]);

        #[cfg(feature = "std")]
        {
            let indexed: Vec<((usize, usize), f64)> =
                mat.iter_indexed().map(|(pos, &val)| (pos, val)).collect();
            assert_eq!(indexed[1], ((0, 1), 2.0));
            assert_eq!(indexed[2], ((1, 0), 3.0));
        }

        for val in mat.iter_mut() {
            *val *= 2.0;
        }
        assert_f64_near!(mat[[1, 1]], 8.0);
    }

    #[test]
    fn test_determinant() {
        let mat2 = Matrix2x2::from_nested_arr([[4.0, 7.0], [2.0, 6.0]]);
        assert_f64_near!(mat2.determinant(), 10.0);
    }

    #[test]
    fn test_indexing() {
        let mut mat = Matrix2x2::zeros();
        mat[[0, 1]] = 42.0;
        assert_f64_near!(mat[[0, 1]], 42.0);
    }

    #[test]
    #[should_panic(expected = "is out of bounds")]
    fn test_index_out_of_bounds() {
        let mat = Matrix2x2::zeros();
        let _ = mat[[2, 0]];
    }
}

#[cfg(test)]
mod tests_3x3 {
    use super::*;

    #[test]
    fn test_mul() {
        let lhs = Matrix3x3::from_nested_arr([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let rhs = Matrix::<3, 1>::from_nested_arr([[10.0], [11.0], [12.0]]);
        let result = lhs * rhs;
        let correct = Matrix::<3, 1>::from_nested_arr([[68.0], [167.0], [266.0]]);
        assert_eq!(result, correct);
    }
}
