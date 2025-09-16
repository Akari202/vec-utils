use std::f64::consts::PI;

/// An angle in degrees
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct AngleDegrees {
    angle: f64
}

/// An angle in radians, f64 is assumed to be in radians
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct AngleRadians {
    /// The angle in radians
    pub angle: f64
}

impl AngleRadians {
    /// Create a new angle in radians
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }

    /// Get 2pi
    pub fn two_pi() -> Self {
        Self::new(2.0 * PI)
    }

    /// Get pi
    pub fn pi() -> Self {
        Self::new(PI)
    }

    /// Get pi/2
    pub fn half_pi() -> Self {
        Self::new(PI / 2.0)
    }

    /// Get pi/4
    pub fn quarter_pi() -> Self {
        Self::new(PI / 4.0)
    }

    /// Get pi/3
    pub fn third_pi() -> Self {
        Self::new(PI / 3.0)
    }

    /// Get pi/6
    pub fn sixth_pi() -> Self {
        Self::new(PI / 6.0)
    }

    /// Get the sine of the angle
    pub fn sin(&self) -> f64 {
        self.angle.sin()
    }

    /// Get the cosine of the angle
    pub fn cos(&self) -> f64 {
        self.angle.cos()
    }

    /// Get the tangent of the angle
    pub fn tan(&self) -> f64 {
        self.angle.tan()
    }

    /// Get the secant of the angle
    pub fn sec(&self) -> f64 {
        1.0 / self.cos()
    }

    /// Get the cosecant of the angle
    pub fn csc(&self) -> f64 {
        1.0 / self.sin()
    }

    /// Get the cotangent of the angle
    pub fn cot(&self) -> f64 {
        1.0 / self.tan()
    }

    /// Get the angle in degrees
    pub fn to_degrees(&self) -> AngleDegrees {
        self.into()
    }

    /// Create a new angle from degrees
    pub fn from_degrees(angle: AngleDegrees) -> Self {
        angle.into()
    }
}

impl AngleDegrees {
    /// Create a new angle in degrees
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }

    /// Get the sine of the angle
    pub fn sin(&self) -> f64 {
        AngleRadians::from_degrees(AngleDegrees::new(self.angle)).sin()
    }

    /// Get the cosine of the angle
    pub fn cos(&self) -> f64 {
        AngleRadians::from_degrees(AngleDegrees::new(self.angle)).cos()
    }

    /// Get the tangent of the angle
    pub fn tan(&self) -> f64 {
        AngleRadians::from_degrees(AngleDegrees::new(self.angle)).tan()
    }

    /// Get the secant of the angle
    pub fn sec(&self) -> f64 {
        1.0 / self.cos()
    }

    /// Get the cosecant of the angle
    pub fn csc(&self) -> f64 {
        1.0 / self.sin()
    }

    /// Get the cotangent of the angle
    pub fn cot(&self) -> f64 {
        1.0 / self.tan()
    }

    /// Get the angle in radians
    pub fn to_radians(&self) -> AngleRadians {
        self.into()
    }

    /// Create a new angle from radians
    pub fn from_radians(angle: AngleRadians) -> Self {
        angle.into()
    }
}

impl From<AngleDegrees> for AngleRadians {
    fn from(value: AngleDegrees) -> Self {
        AngleRadians::new(value.angle * PI / 180.0)
    }
}

impl From<&AngleDegrees> for AngleRadians {
    fn from(value: &AngleDegrees) -> Self {
        AngleRadians::new(value.angle * PI / 180.0)
    }
}

impl From<f64> for AngleRadians {
    fn from(value: f64) -> Self {
        AngleRadians::new(value)
    }
}

impl From<AngleRadians> for AngleDegrees {
    fn from(value: AngleRadians) -> Self {
        AngleDegrees::new(value.angle * 180.0 / PI)
    }
}


impl From<&AngleRadians> for AngleDegrees {
    fn from(value: &AngleRadians) -> Self {
        AngleDegrees::new(value.angle * 180.0 / PI)
    }
}

impl From<AngleRadians> for f64 {
    fn from(value: AngleRadians) -> Self {
        value.angle
    }
}

impl From<&AngleRadians> for f64 {
    fn from(value: &AngleRadians) -> Self {
        value.angle
    }
}

impl std::ops::Div<f64> for AngleRadians {
    type Output = AngleRadians;

    fn div(self, rhs: f64) -> AngleRadians {
        (self.angle / rhs).into()
    }
}

impl std::ops::Mul<f64> for AngleRadians {
    type Output = AngleRadians;

    fn mul(self, rhs: f64) -> AngleRadians {
        (self.angle * rhs).into()
    }
}

impl std::ops::Mul<f64> for AngleDegrees {
    type Output = AngleDegrees;

    fn mul(self, rhs: f64) -> AngleDegrees {
        AngleDegrees::new(self.angle * rhs)
    }
}

impl std::ops::Add<AngleRadians> for AngleRadians {
    type Output = AngleRadians;

    fn add(self, rhs: AngleRadians) -> AngleRadians {
        (self.angle + rhs.angle).into()
    }
}

impl std::ops::Sub<AngleRadians> for AngleRadians {
    type Output = AngleRadians;

    fn sub(self, rhs: AngleRadians) -> AngleRadians {
        (self.angle - rhs.angle).into()
    }
}

impl std::ops::Neg for AngleRadians {
    type Output = AngleRadians;

    fn neg(self) -> AngleRadians {
        (-self.angle).into()
    }
}

impl std::ops::Neg for AngleDegrees {
    type Output = AngleDegrees;

    fn neg(self) -> AngleDegrees {
        AngleDegrees::new(-self.angle)
    }
}

impl std::cmp::Ord for AngleRadians {
    fn cmp(&self, rhs: &AngleRadians) -> std::cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl std::cmp::Ord for AngleDegrees {
    fn cmp(&self, rhs: &AngleDegrees) -> std::cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl std::cmp::Eq for AngleRadians {}

impl std::cmp::Eq for AngleDegrees {}

impl std::fmt::Display for AngleRadians {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.1$} radians", self.angle, precision)
        } else {
            write!(f, "{} radians", self.angle)
        }
    }
}

impl std::fmt::Display for AngleDegrees {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.1$}°", self.angle, precision)
        } else {
            write!(f, "{}°", self.angle)
        }
    }
}

#[cfg(test)]
mod tests {
// TODO
}
