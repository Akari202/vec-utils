use core::ops::Mul;

use matrixmultiply::dgemm;

#[doc(inline)]
use crate::matrix::generic::GMatrix;
use crate::matrix::traits::Zeroable;

/// A generic 2d matrix of width R and height C
pub type Matrix<const R: usize, const C: usize> = GMatrix<R, C, f64>;

/// An alias for 2x2 matracies
pub type Matrix2x2 = Matrix<2, 2>;
/// An alias for 3x3 matracies
pub type Matrix3x3 = Matrix<3, 3>;

impl<const R: usize, const C: usize> Matrix<R, C>
where
    [f64; R * C]: Sized
{
    /// Reduce the matrix to row eschelon form using gaussian elimination and partial pivoting
    pub fn to_ref(&self) -> Self {
        let mut result = *self;
        let mut pivot_row = 0;
        let mut pivot_col = 0;

        while pivot_row < R && pivot_col < C {
            let mut max_row = pivot_row;
            let mut max_val = result[[max_row, pivot_col]].abs();

            for r in (pivot_row + 1)..R {
                let val = result[[r, pivot_col]].abs();
                if val > max_val {
                    max_val = val;
                    max_row = r;
                }
            }

            if max_val.is_zero() {
                pivot_col += 1;
                continue;
            }

            if max_row != pivot_row {
                for c in 0..C {
                    result.values.swap(pivot_row * C + c, max_row * C + c);
                }
            }

            for r in (pivot_row + 1)..R {
                let factor = result[[r, pivot_col]] / result[[pivot_row, pivot_col]];

                result[[r, pivot_col]] = 0.0;

                for c in (pivot_col + 1)..C {
                    let subtracted = factor * result[[pivot_row, c]];
                    result[[r, c]] -= subtracted;
                }
            }

            pivot_row += 1;
            pivot_col += 1;
        }
        result
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
        // Safety: dgemm is an unsafe function
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
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_mul() {
        let lhs = Matrix3x3::from_nested_arr([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let rhs = Matrix::from_nested_arr([[10.0], [11.0], [12.0]]);
        let result = lhs * rhs;
        let correct = Matrix::from_nested_arr([[68.0], [167.0], [266.0]]);
        assert_eq!(result, correct);
    }
}
