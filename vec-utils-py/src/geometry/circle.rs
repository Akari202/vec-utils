use bektor::*;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use super::super::vec3d::Vec3d;
use super::plane::Plane;

#[pyclass(from_py_object)]
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

    #[staticmethod]
    pub fn none() -> Self {
        Circle {
            inner: geometry::circle::Circle::new(&vec3d::Vec3d::zero(), 0.0, &vec3d::Vec3d::zero())
        }
    }

    #[getter]
    pub fn center(&self) -> Vec3d {
        Vec3d {
            inner: self.inner.center
        }
    }

    #[getter]
    pub fn normal(&self) -> Vec3d {
        Vec3d {
            inner: self.inner.normal
        }
    }

    #[getter]
    pub fn radius(&self) -> f64 {
        self.inner.radius
    }

    #[setter]
    pub fn set_radius(&mut self, value: f64) {
        self.inner.radius = value;
    }

    #[setter]
    pub fn set_center(&mut self, value: Vec3d) {
        self.inner.center = value.inner;
    }

    #[setter]
    pub fn set_normal(&mut self, value: Vec3d) {
        self.inner.normal = value.inner;
    }

    #[getter]
    pub fn area(&self) -> f64 {
        self.inner.get_area()
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

    pub fn __eq__(&self, other: &Circle) -> bool {
        self.inner == other.inner
    }

    pub fn __add__<'py>(&self, other: &Bound<'py, PyAny>) -> Self {
        if let Ok(other_vec) = other.extract::<Vec3d>() {
            Circle {
                inner: self.inner + other_vec.inner,
            }
        } else if let Ok(other_f64) = other.extract::<f64>() {
             Circle {
                inner: self.inner + other_f64,
            }
        } else {
            panic!("Unable to add non float or Vec3d to Circle")
        }
    }

    pub fn __sub__<'py>(&self, other: &Bound<'py, PyAny>) -> Self {
        if let Ok(other_vec) = other.extract::<Vec3d>() {
            Circle {
                inner: self.inner - other_vec.inner,
            }
        } else if let Ok(other_f64) = other.extract::<f64>() {
             Circle {
                inner: self.inner - other_f64,
            }
        } else {
            panic!("Unable to subtract non float or Vec3d to Circle")
        }
    }

    pub fn __mul__(&self, rhs: f64) -> Self {
        Circle {
            inner: self.inner * rhs
        }
    }

    pub fn __rmul__(&self, lhs: f64) -> Self {
        Circle {
            inner: lhs * self.inner
        }
    }

    pub fn __truediv__(&self, rhs: f64) -> Self {
        Circle {
            inner: self.inner / rhs
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Circle(center: {}, normal: {}, radius: {})",
            self.inner.center, self.inner.normal, self.inner.radius
        )
    }
}
