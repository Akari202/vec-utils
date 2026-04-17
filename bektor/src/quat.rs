#[allow(unused_imports)]
use core::f64::consts::TAU;
use core::fmt;
use core::ops::{Add, Div, Index, Mul, Sub};
#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(feature = "glam")]
use glam::DQuat;
#[cfg(feature = "nalgebra")]
use nalgebra::Quaternion;
#[cfg(feature = "nalgebra")]
use nalgebra::UnitQuaternion;
#[cfg(feature = "rand")]
use rand::distr::{Distribution, StandardUniform};
#[cfg(feature = "rand")]
use rand::{Rng, RngExt};

use crate::angle::AngleRadians;
#[cfg(feature = "matrix")]
use crate::matrix::real::Matrix3x3;
use crate::vec3d::Vec3d;
use crate::{
    impl_dual_op_variants, impl_single_op_comm, impl_single_op_variants,
    impl_single_op_variants_comm
};

/// A quaternion
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Deserialize, rkyv::Serialize, rkyv::Archive)
)]
#[derive(Debug, Copy, Clone)]
pub struct Quat {
    /// The real, w, component of the quaternion
    pub w: f64,
    /// The i component of the quaternion
    pub i: f64,
    /// The j component of the quaternion
    pub j: f64,
    /// The k component of the quaternion
    pub k: f64
}

impl Quat {
    /// Create a new quaternion
    pub fn new(w: f64, i: f64, j: f64, k: f64) -> Quat {
        Quat { w, i, j, k }
    }

    /// Create a new identity quaternion
    /// i.e. a quaternion with a real component of 1 and imaginary components of 0
    pub fn identity() -> Quat {
        Quat {
            w: 1.0,
            i: 0.0,
            j: 0.0,
            k: 0.0
        }
    }

    /// Create a new Quat from a slice of f64s
    /// the slice should have a length of 4
    /// any additional elements will be ignored
    /// ordering of [w, x, y, z] is assumed
    pub fn from_slice(v: &[f64]) -> Quat {
        Quat {
            w: v[0],
            i: v[1],
            j: v[2],
            k: v[3]
        }
    }

    /// Convert the Quat to a Vec of f64 with length 4
    /// ordering of [w, x, y, z] is assumed
    #[cfg(feature = "std")]
    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.w, self.i, self.j, self.k]
    }

    /// Convert the Quat to a array of f64 with length 4
    /// ordering of [w, x, y, z] is assumed
    pub fn to_array(&self) -> [f64; 4] {
        [self.w, self.i, self.j, self.k]
    }

    /// Create a quaternion from a Vec3d
    /// the x, y, and z components of the vector are used as the imaginary components of the quaternion
    /// the real component of the quaternion is set to 0
    pub fn from_vec3d(v: &Vec3d) -> Quat {
        Quat {
            w: 0.0,
            i: v.x,
            j: v.y,
            k: v.z
        }
    }

    /// Convert the quaternion to a vec3d
    /// the real component of the quaternion is discarded
    /// the imaginary components of the quaternion are used as the vector components
    pub fn to_vec3d(&self) -> Vec3d {
        Vec3d::new(self.i, self.j, self.k)
    }

    /// Create a quaternion from a f64
    /// the real component of the quaternion is set to the float
    /// the imaginary components are set to 0
    pub fn from_f64(num: f64) -> Quat {
        Quat {
            w: num,
            i: 0.0,
            j: 0.0,
            k: 0.0
        }
    }

    /// Convert the quaternion to a f64
    /// extracts the real component the imaginary part is discarded
    pub fn to_f64(&self) -> f64 {
        self.w
    }

    /// Create a new quaternion from an axis and an angle
    /// representing a rotation of the given angle around the given axis
    /// the resulting quaternion is definitionally a unit quaternion
    /// the angle is positive for a counter-clockwise rotation
    pub fn from_axis_angle(axis: &Vec3d, angle: impl Into<AngleRadians>) -> Quat {
        let angle: AngleRadians = angle.into();
        let half_angle: AngleRadians = angle / 2.0;
        let s = half_angle.sin();
        Quat {
            w: half_angle.cos(),
            i: -axis[0] * s,
            j: -axis[1] * s,
            k: -axis[2] * s
        }
    }

    /// Convert the quaternion to an axis and an angle
    pub fn to_axis_angle(&self) -> (Vec3d, AngleRadians) {
        if (self.w - 1.0).abs() < f64::EPSILON {
            (Vec3d::i(), 0.0.into())
        } else {
            let a = self.to_vec3d().magnitude();
            #[cfg(not(feature = "std"))]
            let angle = 2.0 * libm::atan2(a, self.w * self.w);
            #[cfg(feature = "std")]
            let angle = 2.0 * a.atan2(self.w * self.w);
            let vec = Vec3d::new(self.i / a, self.j / a, self.k / a);
            (vec, angle.into())
        }
    }

    /// Create a new quaternion from a rotation matrix
    #[cfg(feature = "matrix")]
    pub fn from_rotation_matrix(m: &Matrix3x3) -> Quat {
        #[cfg(not(feature = "std"))]
        let w = core::f64::math::sqrt(1.0 + m[[0, 0]] + m[[1, 1]] + m[[2, 2]]) / 2.0;
        #[cfg(feature = "std")]
        let w = (1.0 + m[[0, 0]] + m[[1, 1]] + m[[2, 2]]).sqrt() / 2.0;
        #[cfg(not(feature = "std"))]
        let i = core::f64::math::sqrt(1.0 + m[[0, 0]] - m[[1, 1]] - m[[2, 2]]) / 2.0;
        #[cfg(feature = "std")]
        let i = (1.0 + m[[0, 0]] - m[[1, 1]] - m[[2, 2]]).sqrt() / 2.0;
        #[cfg(not(feature = "std"))]
        let j = core::f64::math::sqrt(1.0 - m[[0, 0]] + m[[1, 1]] - m[[2, 2]]) / 2.0;
        #[cfg(feature = "std")]
        let j = (1.0 - m[[0, 0]] + m[[1, 1]] - m[[2, 2]]).sqrt() / 2.0;
        #[cfg(not(feature = "std"))]
        let k = core::f64::math::sqrt(1.0 - m[[0, 0]] - m[[1, 1]] + m[[2, 2]]) / 2.0;
        #[cfg(feature = "std")]
        let k = (1.0 - m[[0, 0]] - m[[1, 1]] + m[[2, 2]]).sqrt() / 2.0;
        if w > i && w > j && w > k {
            Quat {
                w,
                i: (m[[2, 1]] - m[[1, 2]]) / (4.0 * w),
                j: (m[[0, 2]] - m[[2, 0]]) / (4.0 * w),
                k: (m[[1, 0]] - m[[0, 1]]) / (4.0 * w)
            }
        } else if i > j && i > k {
            Quat {
                w: (m[[2, 1]] - m[[1, 2]]) / (4.0 * i),
                i,
                j: (m[[0, 1]] + m[[1, 0]]) / (4.0 * i),
                k: (m[[0, 2]] + m[[2, 0]]) / (4.0 * i)
            }
        } else if j > k {
            Quat {
                w: (m[[0, 2]] - m[[2, 0]]) / (4.0 * j),
                i: (m[[0, 1]] + m[[1, 0]]) / (4.0 * j),
                j,
                k: (m[[1, 2]] + m[[2, 1]]) / (4.0 * j)
            }
        } else {
            Quat {
                w: (m[[1, 0]] - m[[0, 1]]) / (4.0 * k),
                i: (m[[0, 2]] + m[[2, 0]]) / (4.0 * k),
                j: (m[[1, 2]] + m[[2, 1]]) / (4.0 * k),
                k
            }
        }
    }

    #[cfg(feature = "matrix")]
    #[cfg(feature = "std")]
    fn from_rotation_matrix_sarabandi_thomas(m: &Matrix3x3) -> Quat {
        let eta = 0.0;
        Quat {
            w: if m[[1, 1]] + m[[2, 2]] + m[[3, 3]] > eta {
                0.5 * (1.0 + m[[1, 1]] + m[[2, 2]] + m[[3, 3]]).sqrt()
            } else {
                0.5 * (((m[[3, 2]] - m[[2, 3]]).powi(2)
                    + (m[[1, 3]] - m[[3, 1]]).powi(2)
                    + (m[[2, 1]] - m[[1, 2]]).powi(2))
                    / (3.0 - m[[1, 1]] - m[[2, 2]] - m[[3, 3]]))
                .sqrt()
            },
            i: (m[[3, 2]] - m[[2, 3]]).signum()
                * if m[[1, 1]] - m[[2, 2]] - m[[3, 3]] > eta {
                    0.5 * (1.0 + m[[1, 1]] - m[[2, 2]] - m[[3, 3]]).sqrt()
                } else {
                    0.5 * (((m[[3, 2]] - m[[2, 3]]).powi(2)
                        + (m[[1, 2]] + m[[2, 1]]).powi(2)
                        + (m[[3, 1]] + m[[1, 3]]).powi(2))
                        / (3.0 - m[[1, 1]] + m[[2, 2]] + m[[3, 3]]))
                    .sqrt()
                },
            j: (m[[1, 3]] - m[[3, 1]]).signum()
                * if -m[[1, 1]] + m[[2, 2]] - m[[3, 3]] > eta {
                    0.5 * (1.0 - m[[1, 1]] + m[[2, 2]] - m[[3, 3]]).sqrt()
                } else {
                    0.5 * (((m[[1, 3]] - m[[3, 1]]).powi(2)
                        + (m[[1, 2]] + m[[2, 1]]).powi(2)
                        + (m[[2, 3]] + m[[3, 2]]).powi(2))
                        / (3.0 + m[[1, 1]] - m[[2, 2]] + m[[3, 3]]))
                    .sqrt()
                },
            k: (m[[2, 1]] - m[[1, 2]]).signum()
                * if -m[[1, 1]] - m[[2, 2]] + m[[3, 3]] > eta {
                    0.5 * (1.0 - m[[1, 1]] - m[[2, 2]] + m[[3, 3]]).sqrt()
                } else {
                    0.5 * (((m[[2, 1]] - m[[1, 2]]).powi(2)
                        + (m[[3, 1]] + m[[1, 3]]).powi(2)
                        + (m[[3, 2]] + m[[2, 3]]).powi(2))
                        / (3.0 + m[[1, 1]] + m[[2, 2]] - m[[3, 3]]))
                    .sqrt()
                }
        }
    }

    #[cfg(feature = "matrix")]
    #[cfg(feature = "std")]
    fn from_rotation_matrix_markley(m: &Matrix3x3) -> Quat {
        todo!()
    }

    #[cfg(feature = "matrix")]
    #[cfg(feature = "std")]
    fn from_rotation_matrix_shepperd(m: &Matrix3x3) -> Quat {
        todo!()
    }

    #[cfg(feature = "matrix")]
    #[cfg(feature = "std")]
    fn from_rotation_matrix_wu(m: &Matrix3x3) -> Quat {
        todo!()
    }

    /// Convert the quaternion to a rotation matrix
    #[cfg(feature = "matrix")]
    pub fn to_rotation_matrix(&self) -> Matrix3x3 {
        Matrix3x3::from_nested_arr([
            [
                1.0 - 2.0 * (self.j * self.j + self.k * self.k),
                2.0 * (self.i * self.j - self.k * self.w),
                2.0 * (self.i * self.k + self.j * self.w)
            ],
            [
                2.0 * (self.i * self.j + self.k * self.w),
                1.0 - 2.0 * (self.i * self.i + self.k * self.k),
                2.0 * (self.j * self.k - self.i * self.w)
            ],
            [
                2.0 * (self.i * self.k - self.j * self.w),
                2.0 * (self.j * self.k + self.i * self.w),
                1.0 - 2.0 * (self.i * self.i + self.j * self.j)
            ]
        ])
    }

    /// Calculate the conjugate of the quaternion
    /// i.e. the quaternion with the same real component and negated imaginary components
    pub fn conjugate(&self) -> Quat {
        Quat {
            w: self.w,
            i: -self.i,
            j: -self.j,
            k: -self.k
        }
    }

    /// Calculate the multiplicative inverse of the quaternion
    /// q^-1 = q^* / ||q||^2
    pub fn inverse(&self) -> Quat {
        let denom = self.w * self.w + self.i * self.i + self.j * self.j + self.k * self.k;
        Quat {
            w: self.w / denom,
            i: -self.i / denom,
            j: -self.j / denom,
            k: -self.k / denom
        }
    }

    /// Calculate the norm of the quaternion
    /// sometimes called the magnitude
    #[deprecated(since = "0.3.3", note = "magnitude was changed to norm")]
    pub fn magnitude(&self) -> f64 {
        self.norm()
    }

    /// Calculate the norm of the quaternion
    /// sometimes called the magnitude
    pub fn norm(&self) -> f64 {
        #[cfg(not(feature = "std"))]
        return core::f64::math::sqrt(
            self.w * self.w + self.i * self.i + self.j * self.j + self.k * self.k
        );
        #[cfg(feature = "std")]
        return (self.w * self.w + self.i * self.i + self.j * self.j + self.k * self.k).sqrt();
    }

    /// Return a new Quat of the normalized quaternion
    pub fn normalize(&self) -> Quat {
        let magnitude = self.norm();
        Quat {
            w: self.w / magnitude,
            i: self.i / magnitude,
            j: self.j / magnitude,
            k: self.k / magnitude
        }
    }

    /// Check if the quaternion is a unit quaternion
    pub fn is_unit(&self) -> bool {
        (self.norm() - 1.0).abs() < f64::EPSILON
    }

    /// Calculates the dot, or inner, product of two quaternions
    pub fn dot(&self, other: &Quat) -> Quat {
        Quat {
            w: self.w * other.w,
            i: self.i * other.i,
            j: self.j * other.j,
            k: self.k * other.k
        }
    }

    /// Calculates the angular distance between two quaternions
    pub fn angular_distance(&self, other: &Quat) -> AngleRadians {
        let a = self * other.conjugate();
        #[cfg(not(feature = "std"))]
        return AngleRadians::new(2.0 * libm::atan2(a.to_vec3d().magnitude(), a.to_f64()));
        #[cfg(feature = "std")]
        return AngleRadians::new(2.0 * a.to_vec3d().magnitude().atan2(a.to_f64()));
    }

    /// Rotate a vector by the quaternion
    pub fn rotate(&self, v: &Vec3d) -> Vec3d {
        (self * (v.to_quat() * self.conjugate())).to_vec3d()
    }

    /// Interpolates between two rotations with constant angular velocity
    /// Follows the great arc on a sphere of rotations
    pub fn slerp(&self, other: &Quat, t: f64) -> Quat {
        (other * self.conjugate()).powf(t) * self
    }

    /// Straight line interpolates between two rotations
    /// the interpolation is linear in quaternion space
    /// the angular velocity peaks in the middle
    pub fn lerp(&self, other: &Quat, t: f64) -> Quat {
        self * (1.0 - t) + other * t
    }

    /// Takes the quaternion to a power
    /// q^p
    pub fn pow(&self, p: &Quat) -> Quat {
        (self.ln() * p).exp()
    }

    /// Takes the quaternion to a scalar power
    /// q^n
    pub fn powf(&self, n: f64) -> Quat {
        self.pow(&Quat::from_f64(n))
    }

    /// Calculates the natural log of the quaternion
    /// ln(q)
    pub fn ln(&self) -> Quat {
        let a = self.to_vec3d().magnitude();
        let norm = self.norm();
        #[cfg(not(feature = "std"))]
        let b = libm::log(norm);
        #[cfg(feature = "std")]
        let b = norm.ln();
        #[cfg(not(feature = "std"))]
        let c = libm::acos(self.w / norm);
        #[cfg(feature = "std")]
        let c = (self.w / norm).acos();
        Quat {
            w: b,
            i: c * self.i / a,
            j: c * self.j / a,
            k: c * self.k / a
        }
    }

    /// Calculates e to the quaternion
    /// e^q
    pub fn exp(&self) -> Quat {
        let a = self.to_vec3d().magnitude();
        #[cfg(not(feature = "std"))]
        let c = libm::cos(a);
        #[cfg(feature = "std")]
        let c = a.cos();
        #[cfg(not(feature = "std"))]
        let s = libm::sin(a);
        #[cfg(feature = "std")]
        let s = a.sin();
        #[cfg(not(feature = "std"))]
        let e = libm::exp(self.w);
        #[cfg(feature = "std")]
        let e = self.w.exp();
        Quat {
            w: e * c,
            i: e * s * self.i / a,
            j: e * s * self.j / a,
            k: e * s * self.k / a
        }
    }

    // https://stackoverflow.com/questions/31600717/how-to-generate-a-random-quaternion-quickly
    /// Generates a uniformly distributed random unit quaternion
    #[cfg(feature = "rand")]
    pub fn random_unit<R: Rng + ?Sized>(rng: &mut R) -> Quat {
        let u1: f64 = rng.random();
        let u2: f64 = rng.random();
        let u3: f64 = rng.random();

        #[cfg(not(feature = "std"))]
        let sqrt_u1 = core::f64::math::sqrt(1.0 - u1);
        #[cfg(feature = "std")]
        let sqrt_u1 = (1.0 - u1).sqrt();

        #[cfg(not(feature = "std"))]
        let sqrt_1_u1 = core::f64::math::sqrt(u1);
        #[cfg(feature = "std")]
        let sqrt_1_u1 = u1.sqrt();

        let theta1 = TAU * u2;
        let theta2 = TAU * u3;

        #[cfg(not(feature = "std"))]
        let (sin_t1, cos_t1) = (libm::sin(theta1), libm::cos(theta1));
        #[cfg(feature = "std")]
        let (sin_t1, cos_t1) = theta1.sin_cos();

        #[cfg(not(feature = "std"))]
        let (sin_t2, cos_t2) = (libm::sin(theta2), libm::cos(theta2));
        #[cfg(feature = "std")]
        let (sin_t2, cos_t2) = theta2.sin_cos();

        Quat {
            w: sqrt_u1 * sin_t1,
            i: sqrt_u1 * cos_t1,
            j: sqrt_1_u1 * sin_t2,
            k: sqrt_1_u1 * cos_t2
        }
    }
}

macro_rules! impl_dual_op {
    ($trait:ident, $method:ident, $op:tt, $T:ty, $description:literal) => {
        impl $trait for $T {
            type Output = $T;

            #[doc = $description]
            fn $method(self, other: $T) -> $T {
                Self { w: self.w $op other.w, i: self.i $op other.i, j: self.j $op other.j, k: self.k $op other.k }
            }
        }

        impl_dual_op_variants!($trait, $method, $T, $description);
    }
}

macro_rules! impl_single_op {
    ($trait:ident, $method:ident, $op:tt, $T:ty, $W:ty, $description:literal) => {
        impl $trait<$W> for $T {
            type Output = $T;

            #[doc = $description]
            fn $method(self, other: $W) -> $T {
                Self { w: self.w $op other, i: self.i $op other, j: self.j $op other, k: self.k $op other }
            }
        }

        impl_single_op_variants!($trait, $method, $T, $W, $description);
    }
}

impl_dual_op!(Add, add, +, Quat, "Add two Quats together comonent-wise");
impl_dual_op!(Sub, sub, -, Quat, "Subtract one Quat from another component-wise");
impl_single_op_comm!(Add, add, +, Quat, f64, "Add a scalar to each component of a Quat");
impl_single_op!(Sub, sub, -, Quat, f64, "Subtract a scalar from each component of a Quat");
impl_single_op_comm!(Mul, mul, *, Quat, f64, "Multiply a Quat by a scalar");
impl_single_op!(Div, div, /, Quat, f64, "Divide a Quat by a scalar");

impl Mul<Quat> for Quat {
    type Output = Quat;

    /// Multiply two quaternions
    /// also known as a Hamilton product
    fn mul(self, other: Quat) -> Quat {
        Quat {
            w: self.w * other.w - self.i * other.i - self.j * other.j - self.k * other.k,
            i: self.w * other.i + self.i * other.w + self.j * other.k - self.k * other.j,
            j: self.w * other.j + self.j * other.w + self.k * other.i - self.i * other.k,
            k: self.w * other.k + self.k * other.w + self.i * other.j - self.j * other.i
        }
    }
}

impl_dual_op_variants!(
    Mul,
    mul,
    Quat,
    "Multiply two quaternions, also known as a Hamilton product"
);

impl Div<Quat> for Quat {
    type Output = Quat;

    /// Divide two quaternions
    /// because quaternion multiplication is not commutative a left hand precedence is assumed
    /// p / q = p q^-1 != q^-1 p
    fn div(self, other: Quat) -> Quat {
        self * other.inverse()
    }
}

impl_dual_op_variants!(
    Div,
    div,
    Quat,
    "Divide two quaternions, because quaternion multiplication is not commutative a left hand precedence is assumed, p / q = p q^-1 != q^-1 p"
);

impl Index<usize> for Quat {
    type Output = f64;

    /// Index into a quaternion
    /// 0 is w, 1 is x, 2 is y, 3 is z
    /// # Panics
    /// if the index is out of bounds
    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.w,
            1 => &self.i,
            2 => &self.j,
            3 => &self.k,
            _ => panic!("Index out of range")
        }
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.w, self.i, self.j, self.k)
    }
}

impl PartialEq for Quat {
    fn eq(&self, other: &Self) -> bool {
        (self.w - other.w).abs() < f64::EPSILON
            && (self.i - other.i).abs() < f64::EPSILON
            && (self.j - other.j).abs() < f64::EPSILON
            && (self.k - other.k).abs() < f64::EPSILON
    }
}

#[cfg(feature = "rand")]
impl Distribution<Quat> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quat {
        Quat {
            w: rng.random(),
            i: rng.random(),
            j: rng.random(),
            k: rng.random()
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f64_near;
    use pretty_assertions::assert_eq;
    #[cfg(feature = "rand")]
    use rand::SeedableRng;
    #[cfg(feature = "rand")]
    use rand::rngs::SmallRng;

    use super::*;

    #[test]
    fn test_new() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_f64_near!(q.w, 1.0);
        assert_f64_near!(q.i, 2.0);
        assert_f64_near!(q.j, 3.0);
        assert_f64_near!(q.k, 4.0);
    }

    #[test]
    fn test_identity() {
        let q = Quat::identity();
        let good = Quat::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(q, good);
    }

    #[test]
    fn test_from_axis_angle() {
        let axis = Vec3d::i();
        let q = Quat::from_axis_angle(&axis, 0.0);
        let good = Quat::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(q, good);
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_from_rotation_matrix() {
        let m = Matrix3x3::from_nested_arr([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        let q = Quat::from_rotation_matrix(&m);
        let good = Quat::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(q, good);
    }

    #[test]
    fn test_conjugate() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let c = q.conjugate();
        let good = Quat::new(1.0, -2.0, -3.0, -4.0);
        assert_eq!(c, good);
    }

    #[test]
    fn test_norm() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_f64_near!(q.norm(), 5.477_225_575_051_661);
    }

    #[test]
    fn test_is_unit() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.is_unit(), false);
    }

    #[test]
    fn test_to_axis_angle() {
        let q = Quat::new(1.0, 0.0, 0.0, 0.0);
        let (axis, angle) = q.to_axis_angle();
        let good = Vec3d::new(1.0, 0.0, 0.0);
        assert_eq!(axis, good);
        assert_eq!(angle, 0.0.into());
    }

    #[test]
    fn test_to_vec3d() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let v = q.to_vec3d();
        let good = Vec3d::new(2.0, 3.0, 4.0);
        assert_eq!(v, good);
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_to_rotation_matrix() {
        let q = Quat::new(1.0, 0.0, 0.0, 0.0);
        let m = q.to_rotation_matrix();
        assert_f64_near!(m[[0, 0]], 1.0);
        assert_f64_near!(m[[0, 1]], 0.0);
        assert_f64_near!(m[[0, 2]], 0.0);
        assert_f64_near!(m[[1, 0]], 0.0);
        assert_f64_near!(m[[1, 1]], 1.0);
        assert_f64_near!(m[[1, 2]], 0.0);
        assert_f64_near!(m[[2, 0]], 0.0);
        assert_f64_near!(m[[2, 1]], 0.0);
        assert_f64_near!(m[[2, 2]], 1.0);
    }

    #[test]
    fn test_rotate() {
        let q = Quat::new(1.0, 0.0, 0.0, 0.0);
        let v = Vec3d::new(1.0, 0.0, 0.0);
        let r = q.rotate(&v);
        let good = Vec3d::new(1.0, 0.0, 0.0);
        assert_eq!(r, good);
    }

    #[test]
    fn test_mul() {
        let q1 = Quat::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quat::new(5.0, 6.0, 7.0, 8.0);
        let q = q1 * q2;
        let good = Quat::new(-60.0, 12.0, 30.0, 24.0);
        assert_eq!(q, good);
    }

    #[test]
    fn test_index() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_f64_near!(q[0], 1.0);
        assert_f64_near!(q[1], 2.0);
        assert_f64_near!(q[2], 3.0);
        assert_f64_near!(q[3], 4.0);
    }

    #[test]
    #[cfg(feature = "rand")]
    fn test_random_is_unit() {
        let mut rng = SmallRng::seed_from_u64(39332);
        for _ in 0..100 {
            assert_f64_near!(Quat::random_unit(&mut rng).norm(), 1.0);
        }
    }

    // https://naif.jpl.nasa.gov/pub/naif/misc/Quaternion_White_Paper/Quaternions_White_Paper.pdf
    #[test]
    fn test_spice_example() {
        // when naif calculates the quaternion in the white paper example they appear to have not halved the angle
        let phi = AngleRadians::new(f64::atan(4.0 / 3.0) * 2.0);
        let r = Vec3d::k();
        let quat = Quat::from_axis_angle(&r, phi);
        assert_eq!(quat, Quat::new(3.0 / 5.0, 0.0, 0.0, -4.0 / 5.0));
        #[cfg(feature = "matrix")]
        assert_eq!(
            quat.to_rotation_matrix(),
            Matrix3x3::from_nested_arr([
                [-7.0 / 25.0, 24.0 / 25.0, 0.0],
                [-24.0 / 25.0, -7.0 / 25.0, 0.0],
                [0.0, 0.0, 1.0]
            ])
        );
        let vec = Vec3d::new(1.0, 1.0, 0.0);
        let rotated = quat.rotate(&vec);
        let correct = Vec3d::new(17.0 / 25.0, -31.0 / 25.0, 0.0);
        assert_f64_near!(rotated[0], correct[0]);
        assert_f64_near!(rotated[1], correct[1]);
        assert_f64_near!(rotated[2], correct[2]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_from_array_vec() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);

        let expected = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(q.to_array(), expected);
        #[cfg(feature = "std")]
        assert_eq!(q.to_vec(), expected.to_vec());
    }

    #[test]
    fn test_from_slice() {
        let data = [1.0, 2.0, 3.0, 4.0];

        let q = Quat::from_slice(&data);
        for (i, j) in q.to_array().into_iter().zip(data) {
            assert_f64_near!(i, j);
        }
    }

    #[test]
    fn test_angular_distance() {
        let q1 = Quat::new(1.0, 0.0, 0.0, 0.0);
        let q2 = Quat::from_axis_angle(&Vec3d::k(), AngleRadians::half_pi());
        assert_eq!(q1.angular_distance(&q2), AngleRadians::half_pi());
    }

    #[test]
    #[cfg(feature = "nalgebra")]
    fn test_nalgebra_interop() {
        let q = Quat {
            i: 1.0,
            j: 2.0,
            k: 3.0,
            w: 4.0
        };
        let nal_q: Quaternion<f64> = q.into();
        let roundtrip: Quat = nal_q.into();

        assert_eq!(nal_q, Quaternion::new(4.0, 1.0, 2.0, 3.0));
        assert_eq!(q, nal_q);
        assert_eq!(roundtrip, q);
    }

    #[test]
    #[cfg(feature = "glam")]
    fn test_glam_interop() {
        let q = Quat {
            i: 1.0,
            j: 2.0,
            k: 3.0,
            w: 4.0
        };
        let glam_q: DQuat = q.into();
        let roundtrip: Quat = glam_q.into();

        assert_eq!(glam_q, DQuat::from_xyzw(1.0, 2.0, 3.0, 4.0));
        assert_eq!(q, glam_q);
        assert_eq!(roundtrip, q);
    }
}

#[cfg(feature = "nalgebra")]
impl From<Quat> for UnitQuaternion<f64> {
    fn from(q: Quat) -> Self {
        UnitQuaternion::new_normalize(Quaternion::new(q.w, q.i, q.j, q.k))
    }
}

#[cfg(feature = "nalgebra")]
impl From<UnitQuaternion<f64>> for Quat {
    fn from(q: UnitQuaternion<f64>) -> Self {
        Self {
            w: q.w,
            i: q.i,
            j: q.j,
            k: q.k
        }
    }
}

#[cfg(feature = "nalgebra")]
impl From<Quaternion<f64>> for Quat {
    fn from(q: Quaternion<f64>) -> Self {
        Self {
            w: q.w,
            i: q.i,
            j: q.j,
            k: q.k
        }
    }
}

#[cfg(feature = "nalgebra")]
impl From<Quat> for Quaternion<f64> {
    fn from(q: Quat) -> Self {
        Quaternion::new(q.w, q.i, q.j, q.k)
    }
}

#[cfg(feature = "nalgebra")]
impl PartialEq<Quaternion<f64>> for Quat {
    fn eq(&self, other: &Quaternion<f64>) -> bool {
        (self.w - other.w).abs() < f64::EPSILON
            && (self.i - other.i).abs() < f64::EPSILON
            && (self.j - other.j).abs() < f64::EPSILON
            && (self.k - other.k).abs() < f64::EPSILON
    }
}

#[cfg(feature = "nalgebra")]
impl PartialEq<UnitQuaternion<f64>> for Quat {
    fn eq(&self, other: &UnitQuaternion<f64>) -> bool {
        self.is_unit()
            && (self.w - other.w).abs() < f64::EPSILON
            && (self.i - other.i).abs() < f64::EPSILON
            && (self.j - other.j).abs() < f64::EPSILON
            && (self.k - other.k).abs() < f64::EPSILON
    }
}

#[cfg(feature = "glam")]
impl From<Quat> for DQuat {
    fn from(q: Quat) -> Self {
        DQuat::from_xyzw(q.i, q.j, q.k, q.w)
    }
}

#[cfg(feature = "glam")]
impl From<DQuat> for Quat {
    fn from(q: DQuat) -> Self {
        Self {
            w: q.w,
            i: q.x,
            j: q.y,
            k: q.z
        }
    }
}

#[cfg(feature = "glam")]
impl PartialEq<DQuat> for Quat {
    fn eq(&self, other: &DQuat) -> bool {
        (self.w - other.w).abs() < f64::EPSILON
            && (self.i - other.x).abs() < f64::EPSILON
            && (self.j - other.y).abs() < f64::EPSILON
            && (self.k - other.z).abs() < f64::EPSILON
    }
}
