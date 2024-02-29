pub struct Quat {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Quat {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quat {
        Quat { w, x, y, z }
    }
}

