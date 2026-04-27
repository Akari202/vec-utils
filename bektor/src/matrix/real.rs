use core::mem::MaybeUninit;
use core::ops::Mul;

#[cfg(feature = "glam")]
use glam::{DMat2, DMat3};
use matrixmultiply::dgemm;
#[cfg(feature = "nalgebra")]
use nalgebra::SMatrix;

#[doc(inline)]
use crate::matrix::generic::GMatrix;
use crate::quat::Quat;
use crate::vec3d::Vec3d;

/// A generic 2d matrix of width R and height C
pub type Matrix<const R: usize, const C: usize> = GMatrix<R, C, f64>;

/// An alias for 2x2 matracies
pub type Matrix2x2 = Matrix<2, 2>;
/// An alias for 3x3 matracies
pub type Matrix3x3 = Matrix<3, 3>;

impl Matrix<3, 1> {
    /// Convert a 3x1 matrix into a Vec3d.
    pub fn to_vec3d(&self) -> Vec3d {
        Vec3d {
            x: self.values[0],
            y: self.values[1],
            z: self.values[2]
        }
    }
}

impl Matrix<1, 3> {
    // TODO: could be zerocopy
    /// Convert a 1x3 matrix into a Vec3d.
    pub fn to_vec3d(&self) -> Vec3d {
        Vec3d {
            x: self.values[0],
            y: self.values[1],
            z: self.values[2]
        }
    }
}

impl Matrix<4, 1> {
    /// Convert a 4x1 matrix into a Quat, the real component is assumed to be first
    pub fn to_quat(&self) -> Quat {
        Quat {
            w: self.values[0],
            i: self.values[1],
            j: self.values[2],
            k: self.values[3]
        }
    }

    /// Convert a 4x1 matrix into a Quat, the real component is last
    pub fn to_quat_last(&self) -> Quat {
        Quat {
            w: self.values[3],
            i: self.values[0],
            j: self.values[1],
            k: self.values[2]
        }
    }
}

impl Matrix<1, 4> {
    // TODO: could be zerocopy
    /// Convert a 1x4 matrix into a Quat, the real component is assumed to be first
    pub fn to_quat(&self) -> Quat {
        Quat {
            w: self.values[0],
            i: self.values[1],
            j: self.values[2],
            k: self.values[3]
        }
    }

    /// Convert a 4x1 matrix into a Quat, the real component is last
    pub fn to_quat_last(&self) -> Quat {
        Quat {
            w: self.values[3],
            i: self.values[0],
            j: self.values[1],
            k: self.values[2]
        }
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
        // Safety: dgemm is an unsafe function and allows C to be uninit when beta is 0.0
        unsafe {
            let mut result_values: [MaybeUninit<f64>; R * C] = MaybeUninit::uninit().into();
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
                result_values.as_mut_ptr().cast::<f64>(),
                C.cast_signed(),
                1
            );
            Matrix {
                values: core::mem::transmute_copy(&result_values)
            }
        }
    }
}

impl<const R: usize, const C: usize> PartialEq for Matrix<R, C>
where
    [f64; R * C]: Sized
{
    fn eq(&self, other: &Self) -> bool {
        self.values
            .iter()
            .zip(other.values.iter())
            .all(|(a, b)| (a - b).abs() <= f64::EPSILON)
    }
}

#[cfg(feature = "glam")]
impl From<Matrix2x2> for DMat2 {
    fn from(m: Matrix2x2) -> Self {
        DMat2::from_cols_array(m.transpose().values.as_slice().try_into().unwrap())
    }
}
#[cfg(feature = "glam")]
impl From<DMat2> for Matrix2x2 {
    fn from(m: DMat2) -> Self {
        Matrix2x2::from_nested_arr(m.to_cols_array_2d()).transpose()
    }
}
#[cfg(feature = "glam")]
impl PartialEq<DMat2> for Matrix2x2 {
    fn eq(&self, other: &DMat2) -> bool {
        (0..2).all(|r| (0..2).all(|c| (self[[r, c]] - other.col(c)[r]).abs() < f64::EPSILON))
    }
}

#[cfg(feature = "glam")]
impl From<Matrix3x3> for DMat3 {
    fn from(m: Matrix3x3) -> Self {
        DMat3::from_cols_array(m.transpose().values.as_slice().try_into().unwrap())
    }
}
#[cfg(feature = "glam")]
impl From<DMat3> for Matrix3x3 {
    fn from(m: DMat3) -> Self {
        Matrix3x3::from_nested_arr(m.to_cols_array_2d()).transpose()
    }
}
#[cfg(feature = "glam")]
impl PartialEq<DMat3> for Matrix3x3 {
    fn eq(&self, other: &DMat3) -> bool {
        (0..3).all(|r| (0..3).all(|c| (self[[r, c]] - other.col(c)[r]).abs() < f64::EPSILON))
    }
}

#[cfg(feature = "nalgebra")]
impl<const R: usize, const C: usize, T> PartialEq<SMatrix<T, R, C>> for GMatrix<R, C, T>
where
    T: nalgebra::Scalar + PartialEq + Copy,
    [T; R * C]: Sized
{
    fn eq(&self, other: &SMatrix<T, R, C>) -> bool {
        (0..R).all(|r| (0..C).all(|c| self[[r, c]] == other[(r, c)]))
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::{assert_f64_near, assert_float_absolute_eq};
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

    #[test]
    fn test_to_vec3d() {
        let col_matrix = Matrix::<3, 1> {
            values: [1.0, 2.0, 3.0]
        };
        let vec_from_col = col_matrix.to_vec3d();

        assert_f64_near!(vec_from_col.x, 1.0);
        assert_f64_near!(vec_from_col.y, 2.0);
        assert_f64_near!(vec_from_col.z, 3.0);

        let row_matrix = Matrix::<1, 3> {
            values: [4.0, 5.0, 6.0]
        };
        let vec_from_row = row_matrix.to_vec3d();

        assert_f64_near!(vec_from_row.x, 4.0);
        assert_f64_near!(vec_from_row.y, 5.0);
        assert_f64_near!(vec_from_row.z, 6.0);
    }

    #[test]
    fn test_largest_eigenvector_basic() {
        let matrix = Matrix::<2, 2> {
            values: [2.0, 0.0, 0.0, 1.0]
        };

        let iterations = 100;
        let eigenvector = matrix.largest_eigenvector(iterations).values;

        assert_f64_near!(eigenvector[0], 1.0, 32);
        assert_float_absolute_eq!(eigenvector[1], 0.0, 1e-12);
    }

    #[test]
    fn test_largest_eigenvector_symmetric() {
        let matrix = Matrix::<2, 2> {
            values: [2.0, 1.0, 1.0, 2.0]
        };

        let iterations = 20;
        let eigenvector = matrix.largest_eigenvector(iterations).values;

        let expected = 0.5f64.sqrt();
        assert_f64_near!(eigenvector[0], expected);
        assert_f64_near!(eigenvector[1], expected);
    }

    #[test]
    #[cfg(feature = "glam")]
    fn test_glam_dmat2_interop() {
        let m = Matrix2x2::from_nested_arr([[1.0, 2.0], [3.0, 4.0]]);
        let glam_m: DMat2 = m.into();
        let roundtrip: Matrix2x2 = glam_m.into();
        assert_eq!(m, glam_m);
        assert_eq!(roundtrip, m);
    }

    #[test]
    #[cfg(feature = "glam")]
    fn test_glam_dmat3_interop() {
        let m = Matrix3x3::from_nested_arr([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let glam_m: DMat3 = m.into();
        let roundtrip: Matrix3x3 = glam_m.into();
        assert_eq!(m, glam_m);
        assert_eq!(roundtrip, m);
    }

    #[test]
    #[cfg(feature = "nalgebra")]
    fn test_nalgebra_interop() {
        let m = Matrix::<2, 3> {
            values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        };
        let nal_m: SMatrix<f64, 2, 3> = m.into();
        let roundtrip: Matrix<2, 3> = nal_m.into();
        assert_eq!(m, nal_m);
        assert_eq!(roundtrip, m);
    }
}
