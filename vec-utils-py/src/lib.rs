use pyo3::prelude::*;

mod angle;
mod quat;
mod vec3d;

#[pymodule]
mod vec_utils_py {
    #[pymodule_export]
    use crate::angle::AngleRadians;
    #[pymodule_export]
    use crate::quat::Quat;
    #[pymodule_export]
    use crate::vec3d::Vec3d;
}
