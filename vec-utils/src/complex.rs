/// A complex number
pub struct Complex {
    /// The real part of the complex number
    pub real: f64,
    /// The imaginary part of the complex number
    pub imaginary: f64
}


impl Complex {
    /// Create a new complex number
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex { real, imaginary }
    }

    /// Create a new complex number from the square root of a real number
    pub fn sqrt(num: f64) -> Complex {
        if num < 0.0 {
            Complex::new(0.0, num.abs().sqrt())
        } else {
            Complex::new(num.sqrt(), 0.0)
        }
    }

    /// Get the magnitude of the complex number
    pub fn magnitude(&self) -> f64 {
        (self.real.powi(2) + self.imaginary.powi(2)).sqrt()
    }

    /// Get the conjugate of the complex number
    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imaginary: -self.imaginary
        }
    }
}

impl std::ops::Add<&Complex> for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        &self + &other
    }
}

impl std::ops::Add<&Complex> for Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        &self + other
    }
}

impl std::ops::Sub<&Complex> for &Complex {
    type Output = Complex;

    fn sub(self, other: &Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        &self - &other
    }
}

impl std::ops::Sub<&Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: &Complex) -> Complex {
        &self - other
    }
}

impl std::ops::Index<usize> for Complex {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.real,
            1 => &self.imaginary,
            _ => panic!("Index out of range")
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.imaginary < 0.0 {
            write!(f, "{} - {}i", self.real, self.imaginary.abs())
        } else {
            write!(f, "{} + {}i", self.real, self.imaginary)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Complex::new(1.0, 2.0);
        assert_eq!(c.real, 1.0);
        assert_eq!(c.imaginary, 2.0);
    }

    #[test]
    fn test_sqrt() {
        let c = Complex::sqrt(-16.0);
        assert_eq!(c.real, 0.0);
        assert_eq!(c.imaginary, 4.0);
    }

    #[test]
    fn test_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude(), 5.0);
    }

    #[test]
    fn test_conjugate() {
        let c = Complex::new(1.0, 2.0);
        let conjugate = c.conjugate();
        assert_eq!(conjugate.real, 1.0);
        assert_eq!(conjugate.imaginary, -2.0);
    }

    #[test]
    fn test_add() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let sum = c1 + c2;
        assert_eq!(sum.real, 4.0);
        assert_eq!(sum.imaginary, 6.0);
    }

    #[test]
    fn test_sub() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let diff = c1 - c2;
        assert_eq!(diff.real, -2.0);
        assert_eq!(diff.imaginary, -2.0);
    }
}
