use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use vec_utils::*;

use super::super::vec3d::Vec3d;
use super::circle::Circle;
use super::plane::Plane;

pub struct IntersectionError(geometry::intersection::IntersectionError);

impl From<IntersectionError> for PyErr {
    fn from(error: IntersectionError) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<geometry::intersection::IntersectionError> for IntersectionError {
    fn from(other: geometry::intersection::IntersectionError) -> Self {
        Self(other)
    }
}

#[pyfunction]
pub fn circle_circle(
    circle1: &Circle,
    circle2: &Circle
) -> Result<(Vec3d, Vec3d), IntersectionError> {
    let result = geometry::intersection::circle_circle(&circle1.inner, &circle2.inner);
    match result {
        Err(e) => Err(IntersectionError(e)),
        Ok(i) => Ok((Vec3d { inner: i.0 }, Vec3d { inner: i.1 }))
    }
}

#[pyfunction]
pub fn plane_line(plane: &Plane, a: &Vec3d, b: &Vec3d) -> Vec3d {
    Vec3d {
        inner: geometry::intersection::plane_line(&plane.inner, &a.inner, &b.inner)
    }
    // let result = geometry::intersection::plane_line(&plane.inner, &a.inner, &b.inner);
    // match result {
    //     Err(e) => Err(IntersectionError(e)),
    //     Ok(i) => Ok(Vec3d { inner: i })
    // }
}

#[pyfunction]
pub fn circle_point(circle: &Circle, point: &Vec3d, inner: bool) -> bool {
    geometry::intersection::circle_point(&circle.inner, &point.inner, inner).is_ok()
}

#[pyfunction]
pub fn circle_point_unchecked(circle: &Circle, point: &Vec3d, inner: bool) -> bool {
    geometry::intersection::circle_point_unchecked(&circle.inner, &point.inner, inner).is_ok()
}
