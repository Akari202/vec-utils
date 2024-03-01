use crate::vec3d::Vec3d;

/// A quaternion
pub struct Quat {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Quat {
    /// Create a new quaternion
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quat {
        Quat { w, x, y, z }
    }

    /// Create a new identity quaternion
    /// i.e. a quaternion with a real component of 1 and imaginary components of 0
    pub fn identity() -> Quat {
        Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Create a new quaternion from an axis and an angle
    /// representing a rotation of the given angle around the given axis
    /// the resulting quaternion is definitionally a unit quaternion
    pub fn from_axis_angle(axis: &Vec3d, angle: f64) -> Quat {
        let half_angle = angle / 2.0;
        let s = half_angle.sin();
        Quat {
            w: half_angle.cos(),
            x: axis[0] * s,
            y: axis[1] * s,
            z: axis[2] * s
        }
    }

    /// Create a new quaternion from a rotation matrix
    pub fn from_rotation_matrix(m: &[[f64; 3]; 3]) -> Quat {
        let w = (1.0 + m[0][0] + m[1][1] + m[2][2]).sqrt() / 2.0;
        let x = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() / 2.0;
        let y = (1.0 - m[0][0] + m[1][1] - m[2][2]).sqrt() / 2.0;
        let z = (1.0 - m[0][0] - m[1][1] + m[2][2]).sqrt() / 2.0;
        if w > x && w > y && w > z {
            Quat {
                w,
                x: (m[2][1] - m[1][2]) / (4.0 * w),
                y: (m[0][2] - m[2][0]) / (4.0 * w),
                z: (m[1][0] - m[0][1]) / (4.0 * w)
            }
        } else if x > y && x > z {
            Quat {
                w: (m[2][1] - m[1][2]) / (4.0 * x),
                x,
                y: (m[0][1] + m[1][0]) / (4.0 * x),
                z: (m[0][2] + m[2][0]) / (4.0 * x)
            }
        } else if y > z {
            Quat {
                w: (m[0][2] - m[2][0]) / (4.0 * y),
                x: (m[0][1] + m[1][0]) / (4.0 * y),
                y,
                z: (m[1][2] + m[2][1]) / (4.0 * y)
            }
        } else {
            Quat {
                w: (m[1][0] - m[0][1]) / (4.0 * z),
                x: (m[0][2] + m[2][0]) / (4.0 * z),
                y: (m[1][2] + m[2][1]) / (4.0 * z),
                z
            }
        }
    }

    /// Calculate the conjugate of the quaternion
    /// i.e. the quaternion with the same real component and negated imaginary components
    pub fn conjugate(&self) -> Quat {
        Quat {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }

    /// Calculate the magnitude of the quaternion
    pub fn magnitude(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Check if the quaternion is a unit quaternion
    pub fn is_unit(&self) -> bool {
        self.magnitude() == 1.0
    }

    /// Convert the quaternion to an axis and an angle
    pub fn to_axis_angle(&self) -> (Vec3d, f64) {
        return if self.w == 1.0 {
            (Vec3d::i(), 0.0)
        } else {
            let angle = 2.0 * self.w.acos();
            let s = (angle / 2.0).sin();
            let x = self.x / s;
            let y = self.y / s;
            let z = self.z / s;
            (Vec3d::new(x, y, z), angle)
        }
    }

    /// Convert the quaternion to a vector
    /// the real component of the quaternion is discarded
    /// the imaginary components of the quaternion are used as the vector components
    pub fn to_vec(&self) -> Vec3d {
        Vec3d::new(self.x, self.y, self.z)
    }

    /// Convert the quaternion to a rotation matrix
    pub fn to_rotation_matrix(&self) -> [[f64; 3]; 3] {
        [
            [
                1.0 - 2.0 * (self.y * self.y + self.z * self.z),
                2.0 * (self.x * self.y - self.z * self.w),
                2.0 * (self.x * self.z + self.y * self.w)
            ],
            [
                2.0 * (self.x * self.y + self.z * self.w),
                1.0 - 2.0 * (self.x * self.x + self.z * self.z),
                2.0 * (self.y * self.z - self.x * self.w)
            ],
            [
                2.0 * (self.x * self.z - self.y * self.w),
                2.0 * (self.y * self.z + self.x * self.w),
                1.0 - 2.0 * (self.x * self.x + self.y * self.y)
            ]
        ]
    }

    /// Rotate a vector by the quaternion
    /// this is an active rotation
    pub fn rotate(&self, v: &Vec3d) -> Vec3d {
        let qv = Quat { w: 0.0, x: v.x, y: v.y, z: v.z };
        (self.conjugate() * qv * self).to_vec()
    }
}

impl std::ops::Mul for Quat {
    type Output = Quat;

    /// Multiply two quaternions
    fn mul(self, rhs: Quat) -> Quat {
        self.mul(&rhs)
    }
}

impl std::ops::Mul<&Quat> for Quat {
    type Output = Quat;

    /// Multiply two quaternions
    /// also known as a Hamilton product
    fn mul(self, rhs: &Quat) -> Quat {
        Quat {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl std::ops::Index<usize> for Quat {
    type Output = f64;

    /// Index into a quaternion
    /// 0 is w, 1 is x, 2 is y, 3 is z
    /// Panics if the index is out of bounds
    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.w,
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            _ => panic!("Index out of range")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 2.0);
        assert_eq!(q.y, 3.0);
        assert_eq!(q.z, 4.0);
    }

    #[test]
    fn test_identity() {
        let q = Quat::identity();
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
    }

    #[test]
    fn test_from_axis_angle() {
        let axis = Vec3d::i();
        let q = Quat::from_axis_angle(&axis, 0.0);
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
    }

    #[test]
    fn test_from_rotation_matrix() {
        let m = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ];
        let q = Quat::from_rotation_matrix(&m);
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
    }

    #[test]
    fn test_conjugate() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let c = q.conjugate();
        assert_eq!(c.w, 1.0);
        assert_eq!(c.x, -2.0);
        assert_eq!(c.y, -3.0);
        assert_eq!(c.z, -4.0);
    }

    #[test]
    fn test_magnitude() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.magnitude(), 5.477225575051661);
    }

    #[test]
    fn test_is_unit() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.is_unit(), false);
    }

    #[test]
    fn test_to_axis_angle() {
        let q = Quat::new(1.0, 0.0, 0.0, 0.0);
        let (axis, angle) = q.to_axis_angle();
        assert_eq!(axis.x, 1.0);
        assert_eq!(axis.y, 0.0);
        assert_eq!(axis.z, 0.0);
        assert_eq!(angle, 0.0);
    }

    #[test]
    fn test_to_vec() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let v = q.to_vec();
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 4.0);
    }

    #[test]
    fn test_to_rotation_matrix() {
        let q = Quat::new(1.0, 0.0, 0.0, 0.0);
        let m = q.to_rotation_matrix();
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 0.0);
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[1][0], 0.0);
        assert_eq!(m[1][1], 1.0);
        assert_eq!(m[1][2], 0.0);
        assert_eq!(m[2][0], 0.0);
        assert_eq!(m[2][1], 0.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn test_rotate() {
        let q = Quat::new(1.0, 0.0, 0.0, 0.0);
        let v = Vec3d::new(1.0, 0.0, 0.0);
        let r = q.rotate(&v);
        assert_eq!(r.x, 1.0);
        assert_eq!(r.y, 0.0);
        assert_eq!(r.z, 0.0);
    }

    #[test]
    fn test_mul() {
        let q1 = Quat::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quat::new(5.0, 6.0, 7.0, 8.0);
        let q = q1 * q2;
        assert_eq!(q.w, -60.0);
        assert_eq!(q.x, 12.0);
        assert_eq!(q.y, 30.0);
        assert_eq!(q.z, 24.0);
    }

    #[test]
    fn test_index() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q[0], 1.0);
        assert_eq!(q[1], 2.0);
        assert_eq!(q[2], 3.0);
        assert_eq!(q[3], 4.0);
    }
}



