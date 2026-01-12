use core::f64::consts::PI;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::{cmp, fmt};

use crate::{
    impl_dual_op_variants, impl_single_op_comm, impl_single_op_variants,
    impl_single_op_variants_comm
};

/// An angle in degrees
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Deserialize, rkyv::Serialize, rkyv::Archive)
)]
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct AngleDegrees {
    /// The angle in degrees
    pub angle: f64
}

/// An angle in radians, f64 is assumed to be in radians
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Deserialize, rkyv::Serialize, rkyv::Archive)
)]
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct AngleRadians {
    /// The angle in radians
    pub angle: f64
}

impl AngleRadians {
    /// Create a new angle in radians
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }

    /// Get 0
    pub fn zero() -> Self {
        Self::new(0.0)
    }

    /// Get 2pi
    pub fn two_pi() -> Self {
        Self::new(2.0 * PI)
    }

    /// Get pi
    pub fn pi() -> Self {
        Self::new(PI)
    }

    /// Get pi/2
    pub fn half_pi() -> Self {
        Self::new(PI / 2.0)
    }

    /// Get pi/4
    pub fn quarter_pi() -> Self {
        Self::new(PI / 4.0)
    }

    /// Get pi/3
    pub fn third_pi() -> Self {
        Self::new(PI / 3.0)
    }

    /// Get pi/6
    pub fn sixth_pi() -> Self {
        Self::new(PI / 6.0)
    }

    /// Get the sine of the angle
    pub fn sin(&self) -> f64 {
        #[cfg(not(feature = "std"))]
        return libm::sin(self.angle);
        #[cfg(feature = "std")]
        return self.angle.sin();
    }

    /// Get the cosine of the angle
    pub fn cos(&self) -> f64 {
        #[cfg(not(feature = "std"))]
        return libm::cos(self.angle);
        #[cfg(feature = "std")]
        return self.angle.cos();
    }

    /// Get the tangent of the angle
    pub fn tan(&self) -> f64 {
        #[cfg(not(feature = "std"))]
        return libm::tan(self.angle);
        #[cfg(feature = "std")]
        return self.angle.tan();
    }

    /// Get the secant of the angle
    pub fn sec(&self) -> f64 {
        1.0 / self.cos()
    }

    /// Get the cosecant of the angle
    pub fn csc(&self) -> f64 {
        1.0 / self.sin()
    }

    /// Get the cotangent of the angle
    pub fn cot(&self) -> f64 {
        1.0 / self.tan()
    }

    /// Get the angle in degrees
    pub fn to_degrees(&self) -> AngleDegrees {
        self.into()
    }

    /// Create a new angle from degrees
    pub fn from_degrees(angle: AngleDegrees) -> Self {
        angle.into()
    }

    /// Takes the mod of the angle
    /// "wraps" the angle around back to zero
    pub fn wrap(&self) -> Self {
        self % Self::two_pi()
    }
}

impl AngleDegrees {
    /// Create a new angle in degrees
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }

    /// Get the sine of the angle
    pub fn sin(&self) -> f64 {
        AngleRadians::from_degrees(AngleDegrees::new(self.angle)).sin()
    }

    /// Get the cosine of the angle
    pub fn cos(&self) -> f64 {
        AngleRadians::from_degrees(AngleDegrees::new(self.angle)).cos()
    }

    /// Get the tangent of the angle
    pub fn tan(&self) -> f64 {
        AngleRadians::from_degrees(AngleDegrees::new(self.angle)).tan()
    }

    /// Get the secant of the angle
    pub fn sec(&self) -> f64 {
        1.0 / self.cos()
    }

    /// Get the cosecant of the angle
    pub fn csc(&self) -> f64 {
        1.0 / self.sin()
    }

    /// Get the cotangent of the angle
    pub fn cot(&self) -> f64 {
        1.0 / self.tan()
    }

    /// Get the angle in radians
    pub fn to_radians(&self) -> AngleRadians {
        self.into()
    }

    /// Create a new angle from radians
    pub fn from_radians(angle: AngleRadians) -> Self {
        angle.into()
    }

    /// Takes the mod of the angle
    /// "wraps" the angle around back to zero
    pub fn wrap(&self) -> Self {
        self % Self::from_radians(AngleRadians::two_pi())
    }
}

impl From<AngleDegrees> for AngleRadians {
    fn from(value: AngleDegrees) -> Self {
        AngleRadians::new(value.angle * PI / 180.0)
    }
}

impl From<&AngleDegrees> for AngleRadians {
    fn from(value: &AngleDegrees) -> Self {
        AngleRadians::new(value.angle * PI / 180.0)
    }
}

impl From<f64> for AngleRadians {
    fn from(value: f64) -> Self {
        AngleRadians::new(value)
    }
}

impl From<AngleRadians> for AngleDegrees {
    fn from(value: AngleRadians) -> Self {
        AngleDegrees::new(value.angle * 180.0 / PI)
    }
}

impl From<&AngleRadians> for AngleDegrees {
    fn from(value: &AngleRadians) -> Self {
        AngleDegrees::new(value.angle * 180.0 / PI)
    }
}

impl From<AngleRadians> for f64 {
    fn from(value: AngleRadians) -> Self {
        value.angle
    }
}

impl From<&AngleRadians> for f64 {
    fn from(value: &AngleRadians) -> Self {
        value.angle
    }
}

macro_rules! impl_dual_op {
    ($trait:ident, $method:ident, $op:tt, $T:ty, $description:literal) => {
        impl $trait for $T {
            type Output = $T;

            #[doc = $description]
            fn $method(self, other: $T) -> $T {
                Self { angle: self.angle $op other.angle }
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
                Self { angle: self.angle $op other }
            }
        }

        impl_single_op_variants!($trait, $method, $T, $W, $description);
    }
}

impl_dual_op!(Add, add, +, AngleRadians, "Add two angles together");
impl_dual_op!(Sub, sub, -, AngleRadians, "Subtract one angle from another");
impl_dual_op!(Add, add, +, AngleDegrees, "Add two angles together");
impl_dual_op!(Sub, sub, -, AngleDegrees, "Subtract one angle from another");
// TODO: Make rem always wrap positive
impl_dual_op!(Rem, rem, %, AngleDegrees, "The mod of an angle");
impl_dual_op!(Rem, rem, %, AngleRadians, "The mod of an angle");

impl_single_op_comm!(Add, add, +, AngleRadians, f64, "Add a f64 to an angle as radians");
impl_single_op!(Sub, sub, -, AngleRadians, f64, "Subtract a f64 from an angle as radians");
impl_single_op_comm!(Mul, mul, *, AngleRadians, f64, "Multiply an angle");
impl_single_op!(Div, div, /, AngleRadians, f64, "Divide an angle");
impl_single_op!(Rem, rem, %, AngleRadians, f64, "The mod of an angle");

impl_single_op_comm!(Mul, mul, *, AngleDegrees, f64, "Multiply an angle");
impl_single_op!(Div, div, /, AngleDegrees, f64, "Divide an angle");
impl_single_op!(Rem, rem, %, AngleDegrees, f64, "The mod of an angle");

impl Neg for AngleRadians {
    type Output = AngleRadians;

    /// Negates the angle
    fn neg(self) -> AngleRadians {
        (-self.angle).into()
    }
}

impl Neg for AngleDegrees {
    type Output = AngleDegrees;

    /// Negates the angle
    fn neg(self) -> AngleDegrees {
        AngleDegrees::new(-self.angle)
    }
}

impl cmp::Ord for AngleRadians {
    fn cmp(&self, rhs: &AngleRadians) -> cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl cmp::Ord for AngleDegrees {
    fn cmp(&self, rhs: &AngleDegrees) -> cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl cmp::Eq for AngleRadians {}

impl cmp::Eq for AngleDegrees {}

impl fmt::Display for AngleRadians {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.1$} radians", self.angle, precision)
        } else {
            write!(f, "{} radians", self.angle)
        }
    }
}

impl fmt::Display for AngleDegrees {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.1$}°", self.angle, precision)
        } else {
            write!(f, "{}°", self.angle)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;

    use assert_float_eq::assert_f64_near;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_constants_and_new() {
        assert_f64_near!(AngleRadians::new(PI).angle, PI);
        assert_f64_near!(AngleRadians::zero().angle, 0.0);
        assert_f64_near!(AngleRadians::pi().angle, PI);
        assert_f64_near!(AngleRadians::two_pi().angle, 2.0 * PI);
        assert_f64_near!(AngleRadians::half_pi().angle, PI / 2.0);
        assert_f64_near!(AngleRadians::quarter_pi().angle, PI / 4.0);
        assert_f64_near!(AngleRadians::third_pi().angle, PI / 3.0);
        assert_f64_near!(AngleRadians::sixth_pi().angle, PI / 6.0);
    }

    #[test]
    fn test_conversions() {
        let deg = AngleDegrees::new(180.0);
        let rad = deg.to_radians();
        assert_f64_near!(rad.angle, PI);

        let back_to_deg = rad.to_degrees();
        assert_f64_near!(back_to_deg.angle, 180.0);

        let rad_from: AngleRadians = AngleDegrees::new(90.0).into();
        assert_f64_near!(rad_from.angle, PI / 2.0);

        let f64_from: f64 = AngleRadians::pi().into();
        assert_f64_near!(f64_from, PI);
    }

    #[test]
    fn test_trigonometry() {
        let rad = AngleRadians::pi() / 4.0; // 45 degrees
        let deg = AngleDegrees::new(45.0);

        assert_f64_near!(rad.sin(), (2.0f64).sqrt() / 2.0);
        assert_f64_near!(deg.sin(), rad.sin());

        assert_f64_near!(rad.cos(), (2.0f64).sqrt() / 2.0);
        assert_f64_near!(deg.cos(), rad.cos());

        assert_f64_near!(rad.tan(), 1.0);
        assert_f64_near!(deg.tan(), 1.0);

        assert_f64_near!(AngleRadians::zero().sec(), 1.0);
        assert_f64_near!(AngleRadians::half_pi().csc(), 1.0);
    }

    #[test]
    fn test_wrapping() {
        assert_f64_near!(AngleRadians::new(3.0 * PI).wrap().angle, PI);
        assert_f64_near!(AngleDegrees::new(450.0).wrap().angle, 90.0);

        // TODO: fix wrap
        // assert_f64_near!(AngleRadians::new(-PI).wrap().angle, PI);
        // assert_f64_near!(AngleDegrees::new(-90.0).wrap().angle, 270.0);
    }

    #[test]
    fn test_arithmetic() {
        let a = AngleDegrees::new(100.0);
        let b = AngleDegrees::new(50.0);

        assert_f64_near!((a + b).angle, 150.0);
        assert_f64_near!((a - b).angle, 50.0);
        assert_f64_near!((a * 2.0).angle, 200.0);
        assert_f64_near!((a / 2.0).angle, 50.0);
        assert_f64_near!((-a).angle, -100.0);

        let r = AngleRadians::pi();
        assert_f64_near!((r + PI).angle, 2.0 * PI);
    }

    #[test]
    fn test_ordering_and_equality() {
        let a = AngleDegrees::new(10.0);
        let b = AngleDegrees::new(20.0);
        let c = AngleDegrees::new(10.0);

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, c);

        assert!(AngleRadians::pi() > AngleRadians::half_pi());
    }
}
