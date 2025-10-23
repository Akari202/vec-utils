#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::many_single_char_names,
    clippy::return_self_not_must_use
)]
//! A crate for 3D vector, quaternion, geometry, and matrix operations
//! plus some miscelaneous other common things.
//! This library is not focused on performance although improvments are planned

/// Angles and angle conversions
pub mod angle;
/// Complex number operations and functions
pub mod complex;
/// 3d geometry operations and functions
pub mod geometry;
/// Hilbert curve mapping
pub mod hilbert;
/// Internal macros
pub(crate) mod macros;
/// Functions for working with matrices
/// currently only 2x2, 3x3, and 4x4 matrices are supported
/// with functions for calculating the determinant, minor, and cofactor
pub mod matrix;
/// Quaternion operations and functions
pub mod quat;
/// Units and unit conversions
pub mod units;
/// 3D vector operations and functions
pub mod vec3d;
