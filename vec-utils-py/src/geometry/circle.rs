use super::super::angle::AngleRadians;
use super::super::vec3d::Vec3d;
use super::plane::Plane;
use pyo3::prelude::*;
use vec_utils::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub inner: geometry::circle::Circle
}

#[pymethods]
impl Circle {
    #[new]
    fn new(center: &Vec3d, radius: f64, normal: &Vec3d) -> Self {
        Circle {
            inner: geometry::circle::Circle::new(&center.inner, radius, &normal.inner)
        }
    }

    fn get_plane(&self) -> Plane {
        Plane {
            inner: self.inner.get_plane()
        }
    }

    fn in_same_plane(&self, other: &Circle) -> bool {
        self.inner.in_same_plane(&other.inner)
    }

    fn is_degenerate(&self) -> bool {
        self.inner.is_degenerate()
    }

    fn __eq__(&self, other: &Circle) -> bool {
        self.inner == other.inner
    }
}
