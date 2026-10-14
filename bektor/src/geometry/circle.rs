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
         PI * core::f64::math::powi(self.radius, 2)
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::assert_f64_near;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_circle_new() {
        let center = Vec3d::new(1.0, 2.0, 3.0);
        let unnormalized_normal = Vec3d::new(0.0, 0.0, 4.0);
        
        let circle = Circle::new(&center, -5.0, &unnormalized_normal);

        assert_eq!(circle.center, center);
        assert_f64_near!(circle.radius, 5.0);
        assert_f64_near!(circle.normal.magnitude(), 1.0);
    }

    #[test]
    fn test_get_area() {
        let circle = Circle::new(&Vec3d::zero(), 4.0, &Vec3d::i());

        let expected_area = core::f64::consts::PI * 16.0;
        assert_f64_near!(circle.get_area() , expected_area);
    }

    #[test]
    fn test_is_degenerate() {
        let center = Vec3d::zero();
        let normal = Vec3d::i();
        
        let valid_circle = Circle::new(&center, 3.0, &normal);
        let degenerate_circle = Circle::new(&center, 0.0, &normal);

        assert!(!valid_circle.is_degenerate());
        assert!(degenerate_circle.is_degenerate());
    }

    #[test]
    fn test_in_same_plane() {
        let c1 = Circle::new(&Vec3d::zero(), 2.0, &Vec3d::k());
        let c2 = Circle::new(&Vec3d::new(5.0, 3.0, 0.0), 3.0, &Vec3d::k());
        let c3 = Circle::new(&Vec3d::zero(), 2.0, &-Vec3d::k()); 
        let c4 = Circle::new(&Vec3d::k(), 2.0, &Vec3d::k()); 

        assert!(c1.in_same_plane(&c2));
        assert!(c1.in_same_plane(&c3));
        assert!(!c1.in_same_plane(&c4));
    }

    #[test]
    fn test_operators() {
        let circle = Circle::new(&Vec3d::new(1.0, 2.0, 3.0), 4.0, &Vec3d::k());

        let c_added_center = circle + Vec3d::new(1.0, 1.0, 1.0);
        assert_eq!(c_added_center.center, Vec3d::new(2.0, 3.0, 4.0));
        assert_f64_near!(c_added_center.radius, 4.0);

        let c_sub_center = circle - Vec3d::new(1.0, 1.0, 1.0);
        assert_eq!(c_sub_center.center, Vec3d::new(0.0, 1.0, 2.0));

        let c_added_radius = circle + 2.0;
        assert_f64_near!(c_added_radius.radius, 6.0);
        assert_eq!(c_added_radius.center, circle.center);

        let c_sub_radius = circle - 1.5;
        assert_f64_near!(c_sub_radius.radius, 2.5);

        let c_mul = circle * 2.0;
        assert_f64_near!(c_mul.radius, 8.0);
        assert_eq!(c_mul.center, Vec3d::new(2.0, 4.0, 6.0));

        let c_div = circle / 2.0;
        assert_f64_near!(c_div.radius, 2.0);
        assert_eq!(c_div.center, Vec3d::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_partial_eq() {
        let c1 = Circle::new(&Vec3d::zero(), 5.0, &Vec3d::k());
        let c2 = Circle::new(&Vec3d::zero(), 5.0, &Vec3d::k());
        
        let c3 = Circle::new(&Vec3d::zero(), 5.0, &-Vec3d::k()); 
        let c4 = Circle::new(&Vec3d::i(), 5.0, &Vec3d::k());

        assert_eq!(c1, c2);
        assert_eq!(c1, c3);
        assert_ne!(c1, c4);
    }
}
