#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate, clippy::many_single_char_names, clippy::return_self_not_must_use)]
//! A library for 3D vector, quaternion, and matrix operations
//! this library was created because i kept reusing the same code in multiple projects
//! and i wanted to have a single place to maintain and update the code
//! this library is not focused on performance
//! I have since added geometry stuff too!

/// 3D vector operations and functions
pub mod vec3d;
/// quaternion operations and functions
pub mod quat;
/// Functions for working with matrices
/// currently only 2x2, 3x3, and 4x4 matrices are supported
/// with functions for calculating the determinant, minor, and cofactor
pub mod matrix;
/// Complex number operations and functions
pub mod complex;
/// Units and unit conversions
pub mod units;
/// Angles and angle conversions
pub mod angle;
/// 3d geometry operations and functions
pub mod geometry;
