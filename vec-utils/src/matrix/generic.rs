use core::fmt::Debug;
use core::ops::{Index, IndexMut};
#[cfg(feature = "std")]
use std::vec::Vec;

use crate::matrix::traits::{Oneable, Signed, Zeroable};

/// A generic 2d matrix of width R and height C
// TODO: I would like to add a generic is row major switch
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GMatrix<const R: usize, const C: usize, T>
where
    [T; R * C]: Sized
{
    pub(crate) values: [T; R * C]
}

/// An alias for 2x2 matracies
pub type GMatrix2x2<T> = GMatrix<2, 2, T>;
/// An alias for 3x3 matracies
pub type GMatrix3x3<T> = GMatrix<3, 3, T>;

impl<const R: usize, const C: usize, T> GMatrix<R, C, T>
where
    [T; R * C]: Sized,
    T: Debug + Oneable + Zeroable + Copy + Clone + PartialEq + Signed
{
    // const IS_ONE_TALL: bool = R == 1;
    // const IS_ONE_WIDE: bool = C == 1;

    /// Create a matrix from nested vectors.
    /// # Panics
    /// A misshapen vector, i.e. one that's not of length C or that doesnt contain vectors of exclusively length R
    #[cfg(feature = "std")]
    pub fn from_nested_vec(values: Vec<Vec<T>>) -> Self {
        let flattened: Vec<T> = values.into_iter().flatten().collect();
        let values: [T; R * C] = flattened
            .try_into()
            .expect("Input dimensions do not match Matrix size R * C");
        Self { values }
    }

    /// Create a matrix from nested arrays.
    pub fn from_nested_arr(values: [[T; C]; R]) -> Self {
        // Safety: [[T; C]; R] and [T; R * C] have the exact same memory layout
        let flat_values = unsafe {
            let ptr = (&raw const values).cast::<[T; R * C]>();
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
            values: [T::zero(); R * C]
        }
    }

    /// Create a matrix filled with ones
    pub fn ones() -> Self {
        Self {
            values: [T::one(); R * C]
        }
    }

    /// Determines if the matrix is square
    pub fn is_square() -> bool {
        R == C
    }

    /// Counts the number of nonzero values
    pub fn count_nonzero(&self) -> usize {
        self.values
            .iter()
            .fold(0, |acc, i| if i.is_zero() { acc } else { acc + 1 })
    }

    /// Returns the diagonal elements
    #[cfg(feature = "std")]
    pub fn diagonals(&self) -> Vec<T> {
        let min_dimm = R.min(C);
        (0..min_dimm).map(|i| self.values[i + i * C]).collect()
    }

    /// Checks if the matrix is upper triangluar
    /// This does not check if its strictly upper triangluar
    pub fn is_upper_triangular(&self) -> bool {
        for row in 1..R {
            for col in 0..row.min(C) {
                if !self.values[row * C + col].is_zero() {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if the matrix is lower triangluar
    /// This does not check if its strictly lower triangluar
    pub fn is_lower_triangular(&self) -> bool {
        for row in 0..R.min(C) {
            for col in (row + 1)..C {
                if !self.values[row * C + col].is_zero() {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if the matrix is a diagonal matrix
    pub fn is_diagonal(&self) -> bool {
        for row in 0..R {
            for col in 0..row.min(C) {
                if !self.values[row * C + col].is_zero() {
                    return false;
                }
            }
            if row + 1 < C {
                for col in (row + 1)..C {
                    if !self.values[row * C + col].is_zero() {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Iterates over the matrix with enumerated position values
    pub fn iter_indexed(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.values.iter().enumerate().map(|(idx, val)| {
            let r = idx / C;
            let c = idx % C;
            ((r, c), val)
        })
    }

    /// Iterates over the matrix
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    /// Mutably iterates over the matrix
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    /// Mutably iterates over the matrix with enumerated position values
    pub fn iter_indexed_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.values.iter_mut().enumerate().map(|(idx, val)| {
            let r = idx / C;
            let c = idx % C;
            ((r, c), val)
        })
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

impl<const R: usize, const C: usize, T> GMatrix<R, C, T>
where
    [T; R * C]: Sized,
    [T; C * R]: Sized,
    T: Oneable + Zeroable + Copy + Clone
{
    const IS_ONE_DIMM: bool = R == 1 || C == 1;

    /// Transposes the matrix.
    /// For matriacies with a dimension of 1 this is zerocopy.
    /// Otherwise it has to touch every value.
    pub fn transpose(&self) -> GMatrix<C, R, T> {
        if Self::IS_ONE_DIMM {
            // Safety: Both GMatrix<R, C, T> and GMatrix<C, R, T> have the same size and
            // both represent a contiguous strip of memory.
            // transmute_copy is used to move the bits into the new type.
            unsafe {
                let result = core::mem::transmute_copy::<Self, GMatrix<C, R, T>>(self);
                let _ = self;
                result
            }
        } else {
            // TODO: implement blocking for bigger matracies
            let mut output = [T::zero(); C * R];
            for row in 0..R {
                for col in 0..C {
                    output[col * R + row] = self.values[row * C + col];
                }
            }
            GMatrix::<C, R, T> { values: output }
        }
    }
}

impl<const R: usize, const C: usize, T> Index<[usize; 2]> for GMatrix<R, C, T>
where
    [T; R * C]: Sized
{
    type Output = T;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        let [row, col] = idx;
        assert!(
            row < R && col < C,
            "Index [{row}, {col}] is out of bounds for matrix of shape [{R}, {C}]"
        );
        &self.values[row * C + col]
    }
}

impl<const R: usize, const C: usize, T> IndexMut<[usize; 2]> for GMatrix<R, C, T>
where
    [T; R * C]: Sized
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

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f64_near;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::complex::Complex;

    #[test]
    fn test_constructors_f64() {
        let zero = GMatrix2x2::<f64>::zeros();
        let one = GMatrix2x2::<f64>::ones();

        for i in zero.iter() {
            assert_f64_near!(*i, 0.0);
        }
        for i in one.iter() {
            assert_f64_near!(*i, 1.0);
        }

        let arr = GMatrix2x2::from_nested_arr([[1.0, 2.0], [3.0, 4.0]]);
        assert_f64_near!(arr[[0, 0]], 1.0);
        assert_f64_near!(arr[[1, 1]], 4.0);
    }

    #[test]
    fn test_constructors_complex() {
        let c1 = Complex {
            real: 1.0,
            imaginary: 2.0
        };
        let c2 = Complex {
            real: 3.0,
            imaginary: 4.0
        };
        let mat = GMatrix::<1, 2, Complex>::from_nested_arr([[c1, c2]]);

        assert_eq!(mat[[0, 0]], c1);
        assert_eq!(mat[[0, 1]], c2);
    }

    #[test]
    fn test_shape_properties() {
        assert!(GMatrix2x2::<f64>::is_square());
        assert!(!GMatrix::<2, 3, f64>::is_square());

        let mut mat = GMatrix2x2::<f64>::zeros();
        mat[[0, 0]] = 1.0;
        mat[[1, 1]] = 1.0;
        assert_eq!(mat.count_nonzero(), 2);
    }

    #[test]
    fn test_triangular_checks() {
        let ut = GMatrix2x2::<f64>::from_nested_arr([[1.0, 2.0], [0.0, 3.0]]);
        assert!(ut.is_upper_triangular());
        assert!(!ut.is_lower_triangular());

        let lt = GMatrix2x2::<f64>::from_nested_arr([[1.0, 0.0], [2.0, 3.0]]);
        assert!(lt.is_lower_triangular());
        assert!(!lt.is_upper_triangular());

        let diag = GMatrix2x2::<f64>::from_nested_arr([[1.0, 0.0], [0.0, 3.0]]);
        assert!(diag.is_diagonal());
    }

    #[test]
    fn test_iterators() {
        let mut mat = GMatrix2x2::<f64>::from_nested_arr([[1.0, 2.0], [3.0, 4.0]]);

        let indexed: Vec<((usize, usize), f64)> =
            mat.iter_indexed().map(|(pos, &val)| (pos, val)).collect();
        assert_eq!(indexed[1], ((0, 1), 2.0));

        // Mutable iteration
        for ((r, c), val) in mat.iter_indexed_mut() {
            if r == c {
                *val = 0.0;
            }
        }
        assert_f64_near!(mat[[0, 0]], 0.0);
        assert_f64_near!(mat[[1, 1]], 0.0);
    }

    #[test]
    fn test_transpose() {
        // Test Vector (Zero-Copy path)
        let vec = GMatrix::<1, 3, f64>::from_nested_arr([[1.0, 2.0, 3.0]]);
        let vec_t = vec.transpose();
        assert_f64_near!(vec_t[[0, 0]], 1.0);
        assert_f64_near!(vec_t[[2, 0]], 3.0);

        // Test Matrix (Standard path)
        let mat = GMatrix2x2::<f64>::from_nested_arr([[1.0, 2.0], [3.0, 4.0]]);
        let mat_t = mat.transpose();
        assert_f64_near!(mat_t[[0, 1]], 3.0);
        assert_f64_near!(mat_t[[1, 0]], 2.0);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_index_out_of_bounds() {
        let mat = GMatrix2x2::<f64>::zeros();
        let _ = mat[[2, 0]];
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_diagonals() {
        let mat =
            GMatrix3x3::<f64>::from_nested_arr([[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]);
        assert_eq!(mat.diagonals(), vec![1.0, 2.0, 3.0]);
    }
}
