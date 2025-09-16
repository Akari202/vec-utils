use super::super::angle::AngleRadians;
use super::super::vec3d::Vec3d;
use super::circle::Circle;
use pyo3::prelude::*;
use vec_utils::*;


#[pyfunction]
pub fn circle_circle(circle1: &Circle, circle2: &Circle) -> Option<(Vec3d, Vec3d)> {
    let result = geometry::intersection::circle_circle(&circle1.inner, &circle2.inner);
    if let Some(i) = result {
        Some((Vec3d {inner: i.0}, Vec3d {inner: i.1}))
    } else {
        None
    }
}


