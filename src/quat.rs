use crate::angle::AngleRadians;
use crate::vec3d::Vec3d;

/// A quaternion
#[derive(Copy, Clone)]
pub struct Quat {
    /// The real component of the quaternion
    pub w: f64,
    /// The i component of the quaternion
    pub i: f64,
    /// The j component of the quaternion
    pub j: f64,
    /// The k component of the quaternion
    pub k: f64
}

impl Quat {
    /// Create a new quaternion
    pub fn new(w: f64, i: f64, j: f64, k: f64) -> Quat {
        Quat { w, i, j, k }
    }

    /// Create a new identity quaternion
    /// i.e. a quaternion with a real component of 1 and imaginary components of 0
    pub fn identity() -> Quat {
        Quat { w: 1.0, i: 0.0, j: 0.0, k: 0.0 }
    }

    /// Create a new quaternion from an axis and an angle
    /// representing a rotation of the given angle around the given axis
    /// the resulting quaternion is definitionally a unit quaternion
    pub fn from_axis_angle(axis: &Vec3d, angle: impl Into<AngleRadians>) -> Quat {
        let angle: AngleRadians = angle.into();
        let half_angle: AngleRadians = angle / 2.0;
        let s = half_angle.sin();
        Quat {
            w: half_angle.cos(),
            i: axis[0] * s,
            j: axis[1] * s,
            k: axis[2] * s
        }
    }

    /// Create a new quaternion from a rotation matrix
    pub fn from_rotation_matrix(m: &[[f64; 3]; 3]) -> Quat {
        let w = (1.0 + m[0][0] + m[1][1] + m[2][2]).sqrt() / 2.0;
        let i = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() / 2.0;
        let j = (1.0 - m[0][0] + m[1][1] - m[2][2]).sqrt() / 2.0;
        let k = (1.0 - m[0][0] - m[1][1] + m[2][2]).sqrt() / 2.0;
        if w > i && w > j && w > k {
            Quat {
                w,
                i: (m[2][1] - m[1][2]) / (4.0 * w),
                j: (m[0][2] - m[2][0]) / (4.0 * w),
                k: (m[1][0] - m[0][1]) / (4.0 * w)
            }
        } else if i > j && i > k {
            Quat {
                w: (m[2][1] - m[1][2]) / (4.0 * i),
                i,
                j: (m[0][1] + m[1][0]) / (4.0 * i),
                k: (m[0][2] + m[2][0]) / (4.0 * i)
            }
        } else if j > k {
            Quat {
                w: (m[0][2] - m[2][0]) / (4.0 * j),
                i: (m[0][1] + m[1][0]) / (4.0 * j),
                j,
                k: (m[1][2] + m[2][1]) / (4.0 * j)
            }
        } else {
            Quat {
                w: (m[1][0] - m[0][1]) / (4.0 * k),
                i: (m[0][2] + m[2][0]) / (4.0 * k),
                j: (m[1][2] + m[2][1]) / (4.0 * k),
                k
            }
        }
    }

    /// Calculate the conjugate of the quaternion
    /// i.e. the quaternion with the same real component and negated imaginary components
    pub fn conjugate(&self) -> Quat {
        Quat {
            w: self.w,
            i: -self.i,
            j: -self.j,
            k: -self.k
        }
    }

    /// Calculate the magnitude of the quaternion
    pub fn magnitude(&self) -> f64 {
        (self.w * self.w + self.i * self.i + self.j * self.j + self.k * self.k).sqrt()
    }

    /// Check if the quaternion is a unit quaternion
    pub fn is_unit(&self) -> bool {
        self.magnitude() == 1.0
    }

    /// Convert the quaternion to an axis and an angle
    pub fn to_axis_angle(&self) -> (Vec3d, AngleRadians) {
        return if self.w == 1.0 {
            (Vec3d::i(), 0.0.into())
        } else {
            let angle = 2.0 * self.w.acos();
            let s = (angle / 2.0).sin();
            let x = self.i / s;
            let y = self.j / s;
            let z = self.k / s;
            (Vec3d::new(x, y, z), angle.into())
        }
    }

    /// Convert the quaternion to a vector
    /// the real component of the quaternion is discarded
    /// the imaginary components of the quaternion are used as the vector components
    pub fn to_vec(&self) -> Vec3d {
        Vec3d::new(self.i, self.j, self.k)
    }

    /// Convert the quaternion to a rotation matrix
    pub fn to_rotation_matrix(&self) -> [[f64; 3]; 3] {
        [
            [
                1.0 - 2.0 * (self.j * self.j + self.k * self.k),
                2.0 * (self.i * self.j - self.k * self.w),
                2.0 * (self.i * self.k + self.j * self.w)
            ],
            [
                2.0 * (self.i * self.j + self.k * self.w),
                1.0 - 2.0 * (self.i * self.i + self.k * self.k),
                2.0 * (self.j * self.k - self.i * self.w)
            ],
            [
                2.0 * (self.i * self.k - self.j * self.w),
                2.0 * (self.j * self.k + self.i * self.w),
                1.0 - 2.0 * (self.i * self.i + self.j * self.j)
            ]
        ]
    }

    /// Rotate a vector by the quaternion
    /// this is an active rotation
    pub fn rotate(&self, v: &Vec3d) -> Vec3d {
        let qv = Quat { w: 0.0, i: v.x, j: v.y, k: v.z };
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
            w: self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j,
            j: self.w * rhs.j + self.j * rhs.w + self.k * rhs.i - self.i * rhs.k,
            k: self.w * rhs.k + self.k * rhs.w + self.i * rhs.j - self.j * rhs.i
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
            1 => &self.i,
            2 => &self.j,
            3 => &self.k,
            _ => panic!("Index out of range")
        }
    }
}

impl std::fmt::Display for Quat {
    /// Format the quaternion as a string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.w, self.i, self.j, self.k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.w, 1.0);
        assert_eq!(q.i, 2.0);
        assert_eq!(q.j, 3.0);
        assert_eq!(q.k, 4.0);
    }

    #[test]
    fn test_identity() {
        let q = Quat::identity();
        assert_eq!(q.w, 1.0);
        assert_eq!(q.i, 0.0);
        assert_eq!(q.j, 0.0);
        assert_eq!(q.k, 0.0);
    }

    #[test]
    fn test_from_axis_angle() {
        let axis = Vec3d::i();
        let q = Quat::from_axis_angle(&axis, 0.0);
        assert_eq!(q.w, 1.0);
        assert_eq!(q.i, 0.0);
        assert_eq!(q.j, 0.0);
        assert_eq!(q.k, 0.0);
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
        assert_eq!(q.i, 0.0);
        assert_eq!(q.j, 0.0);
        assert_eq!(q.k, 0.0);
    }

    #[test]
    fn test_conjugate() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let c = q.conjugate();
        assert_eq!(c.w, 1.0);
        assert_eq!(c.i, -2.0);
        assert_eq!(c.j, -3.0);
        assert_eq!(c.k, -4.0);
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
        assert_eq!(angle, 0.0.into());
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
        assert_eq!(q.i, 12.0);
        assert_eq!(q.j, 30.0);
        assert_eq!(q.k, 24.0);
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



