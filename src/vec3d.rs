use crate::quat::Quat;

/// A 3D vector
#[derive(Debug, Copy, Clone)]
pub struct Vec3d {
    /// The x component of the vector
    pub x: f64,
    /// The y component of the vector
    pub y: f64,
    /// The z component of the vector
    pub z: f64
}

impl Vec3d {
    /// Create a new Vec3d
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x, y, z }
    }

    /// Create a new Vec3d from a start point to an end point
    pub fn new_from_to(from: &Vec3d, to: &Vec3d) -> Vec3d {
        Vec3d {
            x: to.x - from.x,
            y: to.y - from.y,
            z: to.z - from.z
        }
    }

    /// Create a new Vec3d with all components set to 0
    pub fn zero() -> Vec3d {
        Vec3d { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Create a new Vec3d of the i unit vector
    pub fn i() -> Vec3d {
        Vec3d { x: 1.0, y: 0.0, z: 0.0 }
    }

    /// Create a new Vec3d of the j unit vector
    pub fn j() -> Vec3d {
        Vec3d { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Create a new Vec3d of the k unit vector
    pub fn k() -> Vec3d {
        Vec3d { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Create a new Vec3d from a quaternion
    /// the imaginary components of the quaternion are used as the x, y, and z components of the vector
    /// the real component of the quaternion is ignored
    pub fn from_quat(q: &Quat) -> Vec3d {
        Vec3d {
            x: q.i,
            y: q.j,
            z: q.k
        }
    }

    /// Create a new Vec3d from an array
    pub fn from_array(arr: &[f64; 3]) -> Vec3d {
        Vec3d {
            x: arr[0],
            y: arr[1],
            z: arr[2]
        }
    }

    /// Convert the Vec3d to an array
    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// Convert the Vec3d to a quaternion
    /// the x, y, and z components of the vector are used as the imaginary components of the quaternion
    /// the real component of the quaternion is set to 0
    pub fn to_quat(&self) -> Quat {
        Quat {
            w: 0.0,
            i: self.x,
            j: self.y,
            k: self.z
        }
    }

    /// Create a new Vec3d from a Vec of f64
    /// the Vec should have a length of 3
    /// any additional elements will be ignored
    pub fn from_vec(v: &Vec<f64>) -> Vec3d {
        Vec3d {
            x: v[0],
            y: v[1],
            z: v[2]
        }
    }

    /// Convert the Vec3d to a Vec of f64 with length 3
    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }

    /// Calculate the dot product of two Vec3d
    pub fn dot(&self, other: &Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate the cross product of two Vec3d
    pub fn cross(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    /// Calculate the magnitude of the Vec3d
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Check if the Vec3d is a unit vector
    pub fn is_unit(&self) -> bool {
        self.magnitude() == 1.0
    }

    /// Return a new Vec3d of the normalized vector
    pub fn normalize(&self) -> Vec3d {
        let magnitude = self.magnitude();
        Vec3d {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude
        }
    }

    /// Calculate the angle between two Vec3d's
    /// the result is in radians
    pub fn angle_to(&self, other: &Vec3d) -> f64 {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }

    /// Calculate the scalar triple product of three Vec3d's
    pub fn scalar_triple_product(a: &Vec3d, b: &Vec3d, c: &Vec3d) -> f64 {
        a.dot(&b.cross(&c))
    }

    /// Calculate the distance to another Vec3d
    pub fn distance_to(&self, other: &Vec3d) -> f64 {
        (self - other).magnitude()
    }

    // im not sure if i want to have duplicate functions like this
    // pub fn angle_between(a: &Vec3d, b: &Vec3d) -> f64 {
    //     a.dot(b).acos() / (a.magnitude() * b.magnitude())
    // }

    /// Project a Vec3d onto a plane defined by a normal vector
    /// the normal vector should be a unit vector
    pub fn project_onto_plane(&self, normal: &Vec3d) -> Vec3d {
        self - normal * self.dot(normal)
    }
}

impl std::ops::Add for Vec3d {
    type Output = Vec3d;

    /// Add two Vec3d's together component-wise
    fn add(self, other: Vec3d) -> Vec3d {
        &self + &other
    }
}

impl std::ops::Add<&Vec3d> for Vec3d {
    type Output = Vec3d;

    /// Add two Vec3d's together component-wise
    fn add(self, other: &Vec3d) -> Vec3d {
        &self + other
    }
}

impl std::ops::Add<Vec3d> for &Vec3d {
    type Output = Vec3d;

    /// Add two Vec3d's together component-wise
    fn add(self, other: Vec3d) -> Vec3d {
        self + &other
    }
}

impl std::ops::Add<&Vec3d> for &Vec3d {
    type Output = Vec3d;

    /// Add two Vec3d's together component-wise
    fn add(self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl std::ops::Sub<&Vec3d> for &Vec3d {
    type Output = Vec3d;

    /// Subtract one Vec3d from another component-wise
    fn sub(self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl std::ops::Sub for Vec3d {
    type Output = Vec3d;

    /// Subtract one Vec3d from another component-wise
    fn sub(self, other: Vec3d) -> Vec3d {
        &self - &other
    }
}

impl std::ops::Sub<&Vec3d> for Vec3d {
    type Output = Vec3d;

    /// Subtract one Vec3d from another component-wise
    fn sub(self, other: &Vec3d) -> Vec3d {
        &self - other
    }
}

impl std::ops::Sub<Vec3d> for &Vec3d {
    type Output = Vec3d;

    /// Subtract one Vec3d from another component-wise
    fn sub(self, other: Vec3d) -> Vec3d {
        self - &other
    }
}

impl std::ops::Mul<f64> for Vec3d {
    type Output = Vec3d;

    /// Multiply a Vec3d by a scalar
    fn mul(self, other: f64) -> Vec3d {
        &self * other
    }
}

impl std::ops::Mul<f64> for &Vec3d {
    type Output = Vec3d;

    /// Multiply a Vec3d by a scalar
    fn mul(self, other: f64) -> Vec3d {
        Vec3d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl std::ops::Mul<Vec3d> for f64 {
    type Output = Vec3d;

    /// Multiply a Vec3d by a scalar
    fn mul(self, other: Vec3d) -> Vec3d {
        other * self
    }
}

impl std::ops::Div<f64> for Vec3d {
    type Output = Vec3d;

    /// Divide a Vec3d by a scalar
    fn div(self, other: f64) -> Vec3d {
        Vec3d {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl std::ops::Index<usize> for Vec3d {
    type Output = f64;

    /// Index into a Vec3d
    /// 0 is x, 1 is y, 2 is z
    /// Panics if the index is out of bounds
    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds")
        }
    }
}

impl std::fmt::Display for Vec3d {
    /// Format the Vec3d as a string
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_zero() {
        let v = Vec3d::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_i() {
        let v = Vec3d::i();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_j() {
        let v = Vec3d::j();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_k() {
        let v = Vec3d::k();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn test_new_from_to() {
        let from = Vec3d::new(1.0, 1.0, 1.0);
        let to = Vec3d::new(2.0, 2.0, 2.0);
        let v = Vec3d::new_from_to(&from, &to);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn test_from_quat() {
        let q = Quat::new(1.0, 2.0, 3.0, 4.0);
        let v = Vec3d::from_quat(&q);
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 4.0);
    }

    #[test]
    fn test_from_array() {
        let arr = [1.0, 2.0, 3.0];
        let v = Vec3d::from_array(&arr);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_to_array() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        let arr = v.to_array();
        assert_eq!(arr[0], 1.0);
        assert_eq!(arr[1], 2.0);
        assert_eq!(arr[2], 3.0);
    }

    #[test]
    fn test_to_quat() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        let q = v.to_quat();
        assert_eq!(q.w, 0.0);
        assert_eq!(q.i, 1.0);
        assert_eq!(q.j, 2.0);
        assert_eq!(q.k, 3.0);
    }

    #[test]
    fn test_from_vec() {
        let v = Vec3d::from_vec(&vec![1.0, 2.0, 3.0]);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_to_vec() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        let vec = v.to_vec();
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(4.0, 5.0, 6.0);
        let v = v1.cross(&v2);
        assert_eq!(v.x, -3.0);
        assert_eq!(v.y, 6.0);
        assert_eq!(v.z, -3.0);
    }

    #[test]
    fn test_magnitude() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 3.7416573867739413);
    }

    #[test]
    fn test_is_unit() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(v.is_unit(), false);
    }

    #[test]
    fn test_normalize() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        let n = v.normalize();
        assert_eq!(n.x, 0.2672612419124244);
        assert_eq!(n.y, 0.5345224838248488);
        assert_eq!(n.z, 0.8017837257372732);
    }

    #[test]
    fn test_angle_to() {
        let v1 = Vec3d::k();
        let v2 = Vec3d::i();
        assert_eq!(v1.angle_to(&v2), 1.5707963267948966);
    }

    #[test]
    fn test_scalar_triple_product() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(4.0, 5.0, 6.0);
        let v3 = Vec3d::new(7.0, 8.0, 9.0);
        assert_eq!(Vec3d::scalar_triple_product(&v1, &v2, &v3), 0.0);
    }

    #[test]
    fn test_distance_to() {
        let v1 = Vec3d::new(1.0, 1.0, 1.0);
        let v2 = Vec3d::new(1.0, 1.0, 6.0);
        assert_eq!(v1.distance_to(&v2), 5.0);
    }

    #[test]
    fn test_project_onto_plane() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        let n = Vec3d::new(0.0, 0.0, 1.0);
        let p = v.project_onto_plane(&n);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(4.0, 5.0, 6.0);
        let v = v1 + v2;
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 7.0);
        assert_eq!(v.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = Vec3d::new(4.0, 5.0, 6.0);
        let v = v1 - v2;
        assert_eq!(v.x, -3.0);
        assert_eq!(v.y, -3.0);
        assert_eq!(v.z, -3.0);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v = v1 * 2.0;
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.z, 6.0);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3d::new(1.0, 2.0, 3.0);
        let v = v1 / 2.0;
        assert_eq!(v.x, 0.5);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.5);
    }

    #[test]
    fn test_index() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }
}
