use pyo3::prelude::*;

mod angle;
mod quat;
mod vec3d;
mod geometry;

#[pymodule]
mod vec_utils_py {
    use super::*;
    #[pymodule_export]
    use crate::angle::AngleRadians;
    #[pymodule_export]
    use crate::quat::Quat;
    #[pymodule_export]
    use crate::vec3d::Vec3d;
    #[pymodule]
    mod geometry {
        use super::*;
        #[pymodule_export]
        use crate::geometry::circle::Circle;
        #[pymodule_export]
        use crate::geometry::plane::Plane;
        #[pymodule_export]
        use crate::geometry::intersection::circle_circle;
    }
}
