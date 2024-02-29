pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x, y, z }
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

impl std::ops::Mul for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: f64) -> Vec3d {
        Vec3d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl std::ops::Div for Vec3d {
    type Output = Vec3d;

    fn div(self, other: f64) -> Vec3d {
        Vec3d {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}
