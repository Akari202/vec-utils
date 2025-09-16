use super::super::angle::AngleRadians;
use super::super::vec3d::Vec3d;
use pyo3::prelude::*;
use vec_utils::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub inner: geometry::plane::Plane
}

#[pymethods]
impl Plane {
    #[new]
    fn new(normal: &Vec3d, distance: f64) -> Self {
        Plane {
            inner: geometry::plane::Plane::new(&normal.inner, distance)
        }
    }

    #[staticmethod]
    fn from_point(normal: &Vec3d, point: &Vec3d) -> Plane {
        Plane {
            inner: geometry::plane::Plane::from_point(&normal.inner, &point.inner)
        }
    }

    #[staticmethod]
    fn xy() -> Plane {
        Plane {
            inner: geometry::plane::Plane::xy()
        }
    }

    #[staticmethod]
    fn xz() -> Plane {
        Plane {
            inner: geometry::plane::Plane::xy()
        }
    }

    #[staticmethod]
    fn yz() -> Plane {
        Plane {
            inner: geometry::plane::Plane::xy()
        }
    }

    #[staticmethod]
    fn from_points(point1: &Vec3d, point2: &Vec3d, point3: &Vec3d) -> Plane {
        Plane {
            inner: geometry::plane::Plane::from_points(&point1.inner, &point2.inner, &point3.inner)
        }
    }

    fn distance_to_point(&self, point: &Vec3d) -> f64 {
        self.inner.distance_to_point(&point.inner)
    }
}
