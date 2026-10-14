use core::f64::consts::PI;
use core::ops::{ Mul, Div, Add, Sub };
use crate::{
    impl_single_op_comm, impl_single_op_variants,
    impl_single_op_variants_comm
};

use crate::geometry::plane::Plane;
use crate::vec3d::Vec3d;

/// A circle in 3d space
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Deserialize, rkyv::Serialize, rkyv::Archive)
)]
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    /// The center of the circle
    pub center: Vec3d,
    /// The radius of the circle
    pub radius: f64,
    /// The normal vector of the circle, used to define the plane the circle is on
    pub normal: Vec3d
}

impl Circle {
    /// Create a new circle
    pub fn new(center: &Vec3d, radius: f64, normal: &Vec3d) -> Circle {
        Circle {
            center: *center,
            radius: radius.abs(),
            normal: normal.normalize()
        }
    }

    /// Get the plane the circle is in
    pub fn get_plane(&self) -> Plane {
        Plane::from_point(&self.normal, &self.center)
    }

    /// Get the area of the circle
    pub fn get_area(&self) -> f64 {
        #[cfg(not(feature = "std"))]
        return PI * core::f64::math::powi(self.radius, 2);
        #[cfg(feature = "std")]
        return PI * self.radius.powi(2);
    }

    /// Check if the circle is in the same plane as a second circle
    pub fn in_same_plane(&self, other: &Circle) -> bool {
        if self.normal == other.normal || self.normal == -other.normal {
            let self_distance = -self.normal.dot(&self.center);
            let other_distance = -other.normal.dot(&other.center);
            if (self_distance - other_distance).abs() < f64::EPSILON {
                return true;
            }
        }
        false
    }

    /// Check if the circle is degenerate with a radius of 0
    pub fn is_degenerate(&self) -> bool {
        self.radius == 0.0
    }
}

macro_rules! impl_single_op {
    ($trait:ident, $method:ident, $op:tt, $T:ty, $W:ty, $description:literal) => {
        impl $trait<$W> for $T {
            type Output = $T;

            #[doc = $description]
            fn $method(self, other: $W) -> $T {
                Self { 
                    center: self.center $op other, 
                    radius: self.radius $op other,
                    normal: self.normal
                }
            }
        }

        impl_single_op_variants!($trait, $method, $T, $W, $description);
    }
}

macro_rules! impl_single_op_center {
    ($trait:ident, $method:ident, $op:tt, $T:ty, $W:ty, $description:literal) => {
        impl $trait<$W> for $T {
            type Output = $T;

            #[doc = $description]
            fn $method(self, other: $W) -> $T {
                Self { 
                    center: self.center $op other, 
                    radius: self.radius,
                    normal: self.normal
                }
            }
        }

        impl_single_op_variants!($trait, $method, $T, $W, $description);
    }
}

macro_rules! impl_single_op_radius {
($trait:ident, $method:ident, $op:tt, $T:ty, $W:ty, $description:literal) => {
    impl $trait<$W> for $T {
        type Output = $T;

        #[doc = $description]
        fn $method(self, other: $W) -> $T {
            Self { 
                center: self.center, 
                radius: self.radius $op other,
                normal: self.normal
            }
        }
    }

    impl_single_op_variants!($trait, $method, $T, $W, $description);
}
}



impl_single_op_center!(Add, add, +, Circle, Vec3d, "Add a Vec3d to the center of a Circle");
impl_single_op_center!(Sub, sub, -, Circle, Vec3d, "Subtract a Vec3d to the center of a Circle");
impl_single_op_radius!(Add, add, +, Circle, f64, "Add a scalar to the radius of a Circle");
impl_single_op_radius!(Sub, sub, -, Circle, f64, "Subtract a scalar to the radius of a Circle");

impl_single_op_comm!(Mul, mul, *, Circle, f64, "Multiply a Circle by a scalar");
impl_single_op!(Div, div, /, Circle, f64, "Divide a Circle by a scalar");


impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center
            && self.radius == other.radius
            && (self.normal == other.normal || self.normal == -other.normal)
    }
}
