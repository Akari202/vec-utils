use crate::vec3d::Vec3d;

/// A sphere in space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    /// The center of the sphere
    pub center: Vec3d,
    /// The radius of the sphere
    pub radius: f64
}

impl Sphere {
    /// Create a new sphere
    pub fn new(center: &Vec3d, radius: f64) -> Sphere {
        Sphere {
            center: *center,
            radius: radius.abs()
        }
    }

    /// Get the volume of the sphere
    pub fn volume(&self) -> f64 {
        4.0 / 3.0 * std::f64::consts::PI * self.radius.powi(3)
    }
}
