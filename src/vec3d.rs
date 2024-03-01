use crate::quat::Quat;

pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x, y, z }
    }

    pub fn zero() -> Vec3d {
        Vec3d { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn i() -> Vec3d {
        Vec3d { x: 1.0, y: 0.0, z: 0.0 }
    }

    pub fn j() -> Vec3d {
        Vec3d { x: 0.0, y: 1.0, z: 0.0 }
    }

    pub fn k() -> Vec3d {
        Vec3d { x: 0.0, y: 0.0, z: 1.0 }
    }

    pub fn from_quat(q: &Quat) -> Vec3d {
        Vec3d {
            x: q.x,
            y: q.y,
            z: q.z
        }
    }

    pub fn from_array(arr: &[f64; 3]) -> Vec3d {
        Vec3d {
            x: arr[0],
            y: arr[1],
            z: arr[2]
        }
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn to_quat(&self) -> Quat {
        Quat {
            w: 0.0,
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn from_vec(v: &Vec<f64>) -> Vec3d {
        Vec3d {
            x: v[0],
            y: v[1],
            z: v[2]
        }
    }

    pub fn dot(&self, other: &Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn is_unit(&self) -> bool {
        self.magnitude() == 1.0
    }

    pub fn normalize(&self) -> Vec3d {
        let magnitude = self.magnitude();
        Vec3d {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude
        }
    }

    pub fn angle_to(&self, other: &Vec3d) -> f64 {
        self.dot(other).acos() / (self.magnitude() * other.magnitude())
    }

    pub fn scalar_triple_product(a: &Vec3d, b: &Vec3d, c: &Vec3d) -> f64 {
        a.dot(&b.cross(&c))
    }

    // pub fn angle_between(a: &Vec3d, b: &Vec3d) -> f64 {
    //     a.dot(b).acos() / (a.magnitude() * b.magnitude())
    // }
}

impl std::ops::Add for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl std::ops::Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl std::ops::Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: f64) -> Vec3d {
        Vec3d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl std::ops::Div<f64> for Vec3d {
    type Output = Vec3d;

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

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds")
        }
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
}
